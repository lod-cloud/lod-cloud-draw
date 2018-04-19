extern crate rustimization;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate clap;
extern crate htmlescape;
extern crate noisy_float;

mod data;
mod graph;
mod ident;
mod settings;
mod svg;
mod tree;

use clap::{Arg, App};
use data::Dataset;
use rustimization::minimizer::Funcmin;
use settings::Settings;
use std::collections::HashMap;
use std::fs::File;

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
        .arg(Arg::with_name("repulse_dist")
             .short("d")
             .long("distance")
             .value_name("PIXELS")
             .help("The minimal distance between bubbles")
             .takes_value(true))
        .arg(Arg::with_name("repulse_rigidity")
             .long("repulse-rigidity")
             .value_name("FACTOR")
             .help("The rigidity of repulsion between bubbles")
             .takes_value(true))
        .arg(Arg::with_name("canvas")
             .short("w")
             .long("well")
             .value_name("FORCE")
             .help("The value of the well boundary force")
             .takes_value(true))
        .arg(Arg::with_name("canvas_size")
             .short("c")
             .long("canvas")
             .value_name("PIXELS")
             .help("The radius of the circle that the bubbles should be contained in")
             .takes_value(true))
        .arg(Arg::with_name("canvas_rigidity")
             .long("canvas-rigidity")
             .value_name("FACTOR")
             .help("The rigidity of the well")
             .takes_value(true))
        .arg(Arg::with_name("settings")
             .short("e")
             .long("settings")
             .value_name("settings.json")
             .help("The JSON file containing the settings for the system")
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
Gradient or lbfgsb = Limited BFGS)")
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
             .help("Apply an n x n blocking method to speed up the algorithm 
(default=1, no blocking)")
             .takes_value(true))
        .arg(Arg::with_name("ident")
             .long("ident")
             .value_name("none|neighbour|tags")
             .help("The algorithm used to identify domain (bubble colours) of unidentified datasets"))
        .get_matches();

    let mut model : graph::Model = Default::default();

    model.spring = args.value_of("spring")
        .map(|s| { s.parse::<f64>().expect("Spring force not a decimal") })
        .unwrap_or(0.01);

    model.repulse = args.value_of("repulse")
        .map(|s| { s.parse::<f64>().expect("Repulsion force not a decimal") })
        .unwrap_or(10.0);

    model.repulse_dist = args.value_of("repulse_dist")
        .map(|s| { s.parse::<f64>().expect("Distance of bubbles is not a decimal") })
        .unwrap_or(50.0);

    model.repulse_rigidity = args.value_of("repulse_rigidity")
        .map(|s| { s.parse::<f64>().expect("Repulsion rigidity is not a decimal") })
        .unwrap_or(1.0);

    model.canvas = args.value_of("centre")
        .map(|s| { s.parse::<f64>().expect("Well force not a decimal") })
        .unwrap_or(1.0);

    model.canvas_size = args.value_of("canvas_size")
        .map(|s| { s.parse::<f64>().expect("Canvas size is not a decimal") })
        .unwrap_or(1000.0);

    model.canvas_rigidity = args.value_of("canvas_rigidity")
        .map(|s| { s.parse::<f64>().expect("Canvas rigidity is not a decimal") })
        .unwrap_or(1.0);

    model.n_blocks = args.value_of("n_blocks")
        .map(|s| { s.parse::<usize>().expect("N Blocks not a positive integer") })
        .unwrap_or(1);


    let algorithm = match args.value_of("algorithm") {
        Some("cg") => "cg",
        Some("lbfgsb") => "lbfgsb",
        Some(a) => panic!(format!("{} is not a supported algorithm", a)),
        None => "lbfgsb"
    };

    let ident_algorithm = match args.value_of("ident") {
        Some("none") => "none",
        Some("tags") => "tags",
        Some("neighbour") => "neighbour",
        Some("neighbor") => "neighbour", // For Americans
        Some(a) => panic!(format!("{} is not a supported identification algorithm", a)),
        None => "none"
    };

    let max_iters = args.value_of("max_iters")
        .map(|s| { s.parse::<u32>().expect("Iterations is not an integer") })
        .unwrap_or(10000);

    let settings_filename = args.value_of("settings").unwrap_or("lod-cloud-settings.json");

    let settings_file = File::open(settings_filename).expect("Settings file does not exist");

    let settings : Settings = serde_json::from_reader(settings_file).expect("Settings file is not valid JSON");
    
    let data_filename = args.value_of("data").expect("Data not found (should not be reachable... this is a bug)");

    let data_file = File::open(data_filename).expect("Data file does not exist");

    let mut data : HashMap<String,Dataset> = serde_json::from_reader(data_file).expect("Data contains a JSON error");

    match ident_algorithm {
        "none" => {},
        "neighbour" => ident::domain_by_most_neighbours(&mut data),
        "tags" => ident::domain_by_keywords(&mut data),
        _ => panic!("Unreachable")
    };

    let graph = graph::build_graph(&data, &settings);

    eprintln!("{} nodes in graph", graph.n);

    let f = |x : &Vec<f64>| {
        graph.cost(x, &model)
    };
    let g = |x : &Vec<f64>| {
        graph.zero_fixed_points(
            graph.gradient(x, &model), &settings.fixed_points)
    };

    // 5.0 is constant here that allows the nodes to be placed sufficiently
    // far that the convergence to a good minimum is guaranteed
    let mut x = graph.set_fixed_points(
        tree::build_tree(&graph, model.repulse_dist * 5.0),
        &settings.fixed_points);

    {
        let mut fmin = Funcmin::new(&mut x, &f, &g, algorithm);
        fmin.max_iteration(max_iters);
        fmin.minimize();
    }

    svg::write_graph(&graph, &x, &data, model.canvas_size, &settings,
                     args.value_of("output").expect("Out file not given")).expect("Could not write graph");
}

