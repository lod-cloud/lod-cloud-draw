//! Methods for outputting graphs as SVG
use data::Dataset;
use graph::Graph;
use htmlescape::encode_minimal;
use noisy_float::prelude::*;
use settings::Settings;
use std::cmp::{min, max};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Result,BufWriter,Write};
use std::path::Path;

const LETTER_WIDTH : usize = 17;
const LINE_HEIGHT : usize = 42;

/// Output a graph with a set of locations as an SVG file
pub fn write_graph<P : AsRef<Path>>(graph : &Graph, loc : &Vec<f64>, 
                                    data : &HashMap<String, Dataset>,
                                    well_size : f64,
                                    settings : &Settings, out_file : P) -> Result<()> {

    let mut out = BufWriter::new(File::create(out_file)?);
    let mut abs_max = list_abs_max(&loc) * 1.05;
    if abs_max < well_size {
        abs_max = well_size;
    }
    
    writeln!(&mut out, "<svg
    xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\">",
        (abs_max as usize) * 2, (abs_max as usize) * 2 + LINE_HEIGHT)?;
    writeln!(&mut out, "{}", 
//"  <script xmlns:xlink=\"http://www.w3.org/1999/xlink\" xlink:href=\"http://lod-cloud.net/versions/2017-08-22/SVGPan.js\"/>
"  <script xmlns:xlink=\"http://www.w3.org/1999/xlink\" xlink:href=\"http://lod-cloud.net/versions/2017-08-22/both.js\"/>
  <style>
    circle { 
        stroke: #333;
        stroke-width: 1.5px;
        fill-opacity: 0.8; 
    } 
    .bubble text {
        text-anchor: middle;
        font-size: .5em;
    }
    .circle-active circle { 
        stroke: #e80000;  
        stroke-width: 3px; 
    } 
    .link { 
        stroke: #e0270b;  
        stroke: #555;  
        stroke-opacity: 0.7;  
        stroke-width: 1px;  
        stroke-dasharray:none;
    } 
    .link-activeIncoming {  
        stroke-opacity: 1; 
        stroke-width: 10;  
        stroke: #be1b38; 
    } 
    .link-activeOutgoung {  
        stroke-opacity: 1;  
        stroke-width: 10; 
        stroke: #1AC21D;  
    } 
    .link-activeBoth {  
        stroke-opacity: 1;  
        stroke: #1AC21D;  
        stroke-width: 10;  
        stroke-dasharray:5,10,5;}
  </style>")?;
  writeln!(&mut out, 
"  <g transform=\"scale({})\">", max(r64(0.5),r64(abs_max/1250.0)))?;
  let leg_len = legend_length(settings);
  writeln!(&mut out, "{}",
"    <g id=\"legend\">
      <text transform=\"translate(30,30)\" style=\"font-family:Verdana, Arial;font-size:200%;text-decoration:underline;\">Legend</text>")?;
  let mut i = 45;
  for legend_entry in settings.legend.iter() {
      writeln!(&mut out,
"      <rect width=\"{}\" height=\"35\" style=\"fill:{}\" transform=\"translate(30,{})\"/>
      <text transform=\"translate(35,{})\" style=\"font-family:Verdana, Arial;font-size:200%\">{}</text>", leg_len, legend_entry.colour, i, i + 27, legend_entry.title)?;
      i += 40;
  }
  writeln!(&mut out, "{}",
"    </g>
  </g>
  <g class=\"links\">")?;

    eprintln!("Final well size: {:.3}", abs_max);
    for edge in graph.edges.iter() {
        writeln!(&mut out, "    <line class=\"link\" targetId=\"{}\" sourceId=\"{}\" x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\"/>",
                 edge.src, edge.trg, 
                 loc[edge.src * 2] + abs_max,
                 loc[edge.src * 2 + 1] + abs_max,
                 loc[edge.trg * 2] + abs_max,
                 loc[edge.trg * 2 + 1] + abs_max)?;
    }
    writeln!(&mut out, "  </g>
  <g class=\"nodes\">")?;
    
    for i in 0..graph.n {
        let dataset_name = graph.vertex_name(i).expect("Vertex name not in graph?!"); 
        match data.get(&dataset_name) {
            Some(dataset) => {
                let title = dataset.title.clone()
                    .unwrap_or_else(|| "Unnamed dataset".to_string());
                writeln!(&mut out, 
"    <g id=\"{}\"
        onmouseover=\"mo(this)\" onmouseout=\"mleave(this)\">
      <circle class=\"node\" r=\"{}\" cx=\"{}\" cy=\"{}\" fill=\"{}\"><title>{}</title></circle>
      <a class=\"bubble\" href=\"https://lod-cloud.net/dataset/{}\">
          <text x=\"{}\" y=\"{}\">{}</text>
      </a>
    </g>",
                 i,
                 bubble_size(dataset, settings.bubble_size_factor.unwrap_or(10.0)),
                 loc[i * 2] + abs_max,
                 loc[i * 2 + 1] + abs_max,
                 get_colour(&dataset.domain, &dataset.keywords, settings), 
                 dataset.identifier,
                 dataset.identifier,
                 loc[i * 2] + abs_max,
                 loc[i * 2 + 1] + abs_max,
                 encode_minimal(&shorten_text(&title)))?;
            },
            None => {
                eprintln!("Dataset not in set: {} (maybe `identifier` is incorrect?)", 
                          &dataset_name);
            }
        }
    }

    match settings.rights_text {
        Some(ref rt) =>
            writeln!(&mut out,
                     "    <g transform=\"translate(20,{}) scale({})\">
      <text style=\"font-family: Verdana, Arial;\">{}</text>
    </g>",
    (abs_max as usize) * 2 + LETTER_WIDTH, 
    min(r64(abs_max * 3.0 / ((rt.len() + 1) as f64) / (LETTER_WIDTH as f64)), r64(1.0)),
    rt)?,
        None => {}
    };

    match settings.logo_link {
        Some(ref l) => {
            writeln!(&mut out,
                     "    <image x=\"{}\" y=\"{}\" height=\"{}\" href=\"{}\"/>",
                     (abs_max as usize) * 2 - settings.logo_width.unwrap_or(120),
                     (abs_max as usize) * 2, LINE_HEIGHT, l)?;
        },
        None => {
            writeln!(&mut out,
                     "      <g transform=\"translate({}, {})\">",
                     (abs_max as usize) * 2 - 120,
                     (abs_max as usize) * 2)?;
            writeln!(&mut out,
                     include_str!("by.svg"))?;
            writeln!(&mut out,
                     "      </g>")?;
        }
    };

    writeln!(&mut out, "  </g>
</svg>")
}

fn list_abs_max(xs : &Vec<f64>) -> f64 {
    let mut max = 0.0;
    for x in xs {
        if x.is_finite() && x.abs() > max {
            max = x.abs()
        }
    }
    max
}

fn get_colour(domain : &str, keywords : &Vec<String>, settings : &Settings) -> String {
    for e in settings.legend.iter() {
        if let Some(ref d) = e.domain {
            if domain == d {
                return e.colour.to_string()
            }
        }
        if let Some(ref tags) = e.keywords {
            for t in tags.iter() {
                if keywords.contains(t) {
                    return e.colour.to_string()
                }
            }
            if tags.len() == 0 {
                return e.colour.to_string()
            }
        }
    }
    "white".to_string()
}

fn shorten_text(text : &str) -> String {
    if text.len() > 9 {
        let mut s = text.chars().take(6).collect::<String>();
        s.push_str("...");
        s
    } else {
        text.to_string()
    }
}

fn bubble_size(dataset : &Dataset, factor : f64) -> String {
    let size = (dataset.triples.get() as f64) + 1.0;
    format!("{:.1}", 15.0 + size.log(factor))
}


fn legend_length(settings : &Settings) -> usize {
    let m = settings.legend.iter().map(|le| {
        le.title.len() * LETTER_WIDTH
    }).max().unwrap_or(0);
    max(m, 310)
}
