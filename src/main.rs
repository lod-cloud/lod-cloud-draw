extern crate rustimization;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate clap;

mod data;
mod graph;
mod svg;
mod tree;

use clap::{Arg, App};
use rustimization::minimizer::Funcmin;
use std::fs::File;
use data::Dataset;
use std::collections::HashMap;
use graph::{Graph,Edge};

fn build_graph(data : &HashMap<String, Dataset>) -> Graph {
    let mut g = Graph::new();
    let mut idmap = HashMap::new();
    for dataset in data.values() {
        if !dataset.links.is_empty() {
            let v1 = *idmap.entry(&dataset.identifier).or_insert_with(|| g.add_vertex());
            for link in dataset.links.iter() {
                let v2 = *idmap.entry(&link.target).or_insert_with(|| g.add_vertex());
                g.edges.push(Edge::new(v1,v2));
                g.edges.push(Edge::new(v2,v1));
            }
        }
    }
    g
}

fn main() {
    let args = App::new("LOD cloud diagram SVG creator")
        .version("1.0")
        .author("John P. McCrae <john@mccr.ae>")
        .about("Tool used to create LOD cloud diagrams as SVG.
The cloud is created as a minimization of the following function:

  f(V,E) = s * sum_{e} spring(e) + r * sum_{v1} sum_{v2} repulse(v1, v2, d) + 
                w * sum_{v} well(v, c)

Where:

  spring(e): Measures the length of a link in the cloud
  repulse(v1, v2, d): Indicates if v1 and v2 are within a distance of d
  well(v, c): Indicates if v is contained within a circle (well) of radius c

And s,r,w are tuning constants")
        .arg(Arg::with_name("spring")
             .short("s")
             .long("spring")
             .value_name("FORCE")
             .help("The value of the spring force")
             .takes_value(true))
        .arg(Arg::with_name("repulse")
             .short("r")
             .long("repulse")
             .value_name("FORCE")
             .help("The value of the repulsion force")
             .takes_value(true))
        .arg(Arg::with_name("well")
             .short("w")
             .long("well")
             .value_name("FORCE")
             .help("The value of the well boundary force")
             .takes_value(true))
        .arg(Arg::with_name("smin")
             .short("d")
             .long("distance")
             .value_name("PIXELS")
             .help("The minimal distance between bubbles")
             .takes_value(true))
        .arg(Arg::with_name("canvas")
             .short("c")
             .long("canvas")
             .value_name("PIXELS")
             .help("The radius of the circle that the bubbles should be contained in")
             .takes_value(true))
        .arg(Arg::with_name("well_wall")
             .long("well-wall")
             .value_name("EXPONENT")
             .help("The strength of the well's walls. EXPERT ONLY")
             .takes_value(true))
        .arg(Arg::with_name("data")
             .index(1)
             .required(true)
             .value_name("data.json")
             .help("The data of the LOD cloud")
             .takes_value(true))
         .arg(Arg::with_name("output")
             .index(2)
             .required(true)
             .value_name("output.svg")
             .help("The path of the SVG file to write to")
             .takes_value(true))
        .arg(Arg::with_name("algorithm")
             .long("algorithm")
             .value_name("cg|lbfgsb")
             .help("The algorithm used to find the cloud diagram (cg=Conjugate
Gradietn or lbfgsb = Limited BFGS)")
             .takes_value(true))
        .arg(Arg::with_name("max_iters")
             .short("i")
             .long("max-iters")
             .value_name("ITERATIONS")
             .help("The maximum number of iterations to perform (default=10000)")
             .takes_value(true))
        .arg(Arg::with_name("n_blocks")
             .short("n")
             .long("n-blocks")
             .value_name("BLOCKS")
             .help("Apply an n x n blocking method to speed up the algorithm (default=1, no blocking")
             .takes_value(true))
        .get_matches();

    let spring = args.value_of("spring")
        .map(|s| { s.parse::<f64>().expect("Spring force not a decimal") })
        .unwrap_or(0.001);

    let repulse = args.value_of("repulse")
        .map(|s| { s.parse::<f64>().expect("Repulsion force not a decimal") })
        .unwrap_or(100.0);

    let centre = args.value_of("centre")
        .map(|s| { s.parse::<f64>().expect("Well force not a decimal") })
        .unwrap_or(1.0);

    let n_blocks = args.value_of("n_blocks")
        .map(|s| { s.parse::<usize>().expect("N Blocks not a positive integer") })
        .unwrap_or(1);

    let smin = args.value_of("smin")
        .map(|s| { s.parse::<f64>().expect("Distance of bubbles is not a decimal") })
        .unwrap_or(60.0);

    let canvas = args.value_of("canvas")
        .map(|s| { s.parse::<f64>().expect("Canvas size is not a decimal") })
        .unwrap_or(1000.0);

    let wall = args.value_of("well_wall")
        .map(|s| { s.parse::<f64>().expect("Well wall is not a decimal") })
        .unwrap_or(10.0);

    let algorithm = match args.value_of("algorithm") {
        Some("cg") => "cg",
        Some("lbfgsb") => "lbfgsb",
        Some(a) => panic!(format!("{} is not a supported algorithm", a)),
        None => "lbfgsb"
    };

    let max_iters = args.value_of("max_iters")
        .map(|s| { s.parse::<u32>().expect("Iterations is not an integer") })
        .unwrap_or(10000);

    let data_filename = args.value_of("data").expect("Data not found (should not be reachable... this is a bug)");

    let data_file = File::open(data_filename).expect("Data file does not exist");

    let data : HashMap<String,Dataset> = serde_json::from_reader(data_file).expect("JSON error");

    let graph = build_graph(&data);

    let f = |x : &Vec<f64>| {
        graph.cost2(x, spring, repulse, smin, centre, wall, canvas, n_blocks) 
    };
    let g = |x : &Vec<f64>| {
        graph.gradient2(x, spring, repulse, smin, centre, wall, canvas, n_blocks) 
    };

    // 5.0 is constant here that allows the nodes to be placed sufficiently
    // far that the convergence to a good minimum is guaranteed
    let mut x = tree::build_tree(&graph, smin * 5.0);

    {
        let mut fmin = Funcmin::new(&mut x, &f, &g, algorithm);
        fmin.max_iteration(max_iters);
        fmin.minimize();
    }

    svg::write_graph(&graph, &x, args.value_of("output").expect("Out file not given")).expect("Could not write graph");
}

