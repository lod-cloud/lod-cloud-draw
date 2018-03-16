//! Methods for outputting graphs as SVG
use data::Dataset;
use graph::Graph;
use htmlescape::encode_minimal;
use settings::Settings;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Result,BufWriter,Write};
use std::path::Path;

/// Output a graph with a set of locations as an SVG file
pub fn write_graph<P : AsRef<Path>>(graph : &Graph, loc : &Vec<f64>, 
                                    data : &HashMap<String, Dataset>,
                                    settings : &Settings, out_file : P) -> Result<()> {

    let mut out = BufWriter::new(File::create(out_file)?);
    let abs_max = list_abs_max(&loc) * 1.05;
    
    writeln!(&mut out, "<svg
    xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\">",
        (abs_max as i32) * 2, (abs_max as i32) * 2)?;
    writeln!(&mut out, "{}", 
"  <script xmlns:xlink=\"http://www.w3.org/1999/xlink\" xlink:href=\"http://lod-cloud.net/versions/2017-08-22/SVGPan.js\"/>
  <script xmlns:xlink=\"http://www.w3.org/1999/xlink\" xlink:href=\"http://lod-cloud.net/versions/2017-08-22/both.js\"/>
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
  writeln!(&mut out, "{}",
"  <g transform=\"translate(-460,-390) scale(0.8)\">
    <g id=\"legend\">
      <text transform=\"translate(610,530)\" style=\"font-family:Verdana, Arial;font-size:200%;text-decoration:underline;\">Legend</text>")?;
  let mut i = 545;
  for legend_entry in settings.legend.iter() {
      writeln!(&mut out,
"      <rect width=\"310\" height=\"35\" style=\"fill:{}\" transform=\"translate(600,{})\"/>
      <text transform=\"translate(610,{})\" style=\"font-family:Verdana, Arial;font-size:200%\">{}</text>", legend_entry.colour, i, i + 27, legend_entry.title)?;
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
      <circle class=\"node\" r=\"{}\" cx=\"{}\" cy=\"{}\" fill=\"{}\"/>
      <a class=\"bubble\" href=\"http://www.example.com/{}\">
          <text x=\"{}\" y=\"{}\">{}</text>
      </a>
    </g>",
                 i,
                 bubble_size(dataset),
                 loc[i * 2] + abs_max,
                 loc[i * 2 + 1] + abs_max,
                 get_colour(&dataset.domain, settings), 
                 dataset.identifier,
                 loc[i * 2] + abs_max,
                 loc[i * 2 + 1] + abs_max,
                 encode_minimal(&shorten_text(&title)))?;
            },
            None => {
                eprintln!("Dataset not in set: {}", 
                          &dataset_name);
            }
        }
    }

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

fn get_colour(domain : &str, settings : &Settings) -> String {
    for e in settings.legend.iter() {
        if domain == e.id {
            return e.colour.to_string()
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

fn bubble_size(dataset : &Dataset) -> String {
    let size = (dataset.triples.get() as f64) + 1.0;
    format!("{:.1}", 15.0 + size.log(10.0))
}
