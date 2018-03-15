//! Methods for outputting graphs as SVG
use std::path::Path;
use std::fs::File;
use std::io::{Result,BufWriter,Write};
use graph::Graph;

/// Output a graph with a set of locations as an SVG file
pub fn write_graph<P : AsRef<Path>>(graph : &Graph, loc : &Vec<f64>,
                                    out_file : P) -> Result<()> {

    let mut out = BufWriter::new(File::create(out_file)?);
    
    writeln!(&mut out, "{}", "<svg
    xmlns=\"http://www.w3.org/2000/svg\" width=\"2000\" height=\"2000\">
  <script xmlns:xlink=\"http://www.w3.org/1999/xlink\" xlink:href=\"SVGPan.js\"/>
  <script xmlns:xlink=\"http://www.w3.org/1999/xlink\" xlink:href=\"both.js\"/>
  <style>.circle {  stroke: #333;  stroke-width: 1.5px; fill-opacity: 0.8; } .circle-pasive {  stroke: #333;  stroke-width: 1.5px; fill-opacity: 0.0; } .node {  stroke: #000000; font-size: 18px;}  .node:hover{ stroke-opacity: 1;}  .circle-active { stroke: #e80000;  stroke-width: 3px; fill:#e80000; fill-opacity: 0.3;} .link { stroke: #e0270b;  stroke: #555;  stroke-opacity: 0.7;  stroke-width: 1px;  stroke-dasharray:none;} .link-activeIncoming {  stroke-opacity: 1; stroke-width: 10;  stroke: #be1b38; } .link-activeOutgoung {  stroke-opacity: 1;  stroke-width: 10; stroke: #1AC21D;  } .link-activeBoth {  stroke-opacity: 1;  stroke: #1AC21D;  stroke-width: 10;  stroke-dasharray:5,10,5;}</style>
  <rect width=\"2000\" height=\"2000\" style=\"fill: none; pointer-events: all;\"/>
  <g transform=\"translate(-460,-390) scale(0.8)\">
    <g id=\"legend\">
      <text transform=\"translate(610,530)\" style=\"font-family:Verdana, Arial;font-size:200%;text-decoration:underline;\">Legend</text>
      <rect width=\"310\" height=\"35\" style=\"fill:#c8a788\" transform=\"translate(600,545)\"/>
      <text transform=\"translate(610,572)\" style=\"font-family:Verdana, Arial;font-size:200%\">Cross Domain</text>
      <rect width=\"310\" height=\"35\" style=\"fill:#28c9cc\" transform=\"translate(600,585)\"/>
      <text transform=\"translate(610,612)\" style=\"font-family:Verdana, Arial;font-size:200%\">Geography</text>
      <rect width=\"310\" height=\"35\" style=\"fill:#f6b33c\" transform=\"translate(600,625)\"/>
      <text transform=\"translate(610,652)\" style=\"font-family:Verdana, Arial;font-size:200%\">Government</text>
      <rect width=\"310\" height=\"35\" style=\"fill:#db777f\" transform=\"translate(600,665)\"/>
      <text transform=\"translate(610,692)\" style=\"font-family:Verdana, Arial;font-size:200%\">Life Sciences</text>
      <rect width=\"310\" height=\"35\" style=\"fill:#36bc8d\" transform=\"translate(600,705)\"/>
      <text transform=\"translate(610,732)\" style=\"font-family:Verdana, Arial;font-size:200%\">Linguistics</text>
      <rect width=\"310\" height=\"35\" style=\"fill:#008080\" transform=\"translate(600,745)\"/>
      <text transform=\"translate(610,772)\" style=\"font-family:Verdana, Arial;font-size:200%\">Media</text>
      <rect width=\"310\" height=\"35\" style=\"fill:#f6f3ce\" transform=\"translate(600,785)\"/>
      <text transform=\"translate(610,812)\" style=\"font-family:Verdana, Arial;font-size:200%\">Publications</text>
      <rect width=\"310\" height=\"35\" style=\"fill:#b5b5b5\" transform=\"translate(600,825)\"/>
      <text transform=\"translate(610,852)\" style=\"font-family:Verdana, Arial;font-size:200%\">Social Networking</text>
      <rect width=\"310\" height=\"35\" style=\"fill:#d84d8c\" transform=\"translate(600,865)\"/>
      <text transform=\"translate(610,892)\" style=\"font-family:Verdana, Arial;font-size:200%\">User Generated</text>
      <line class=\"link-activeIncoming\" x1=\"600\" y1=\"920\" x2=\"700\" y2=\"920\" style=\"stroke-width:12px;\" targetId=\"0\" sourceId=\"0\"/>
      <text transform=\"translate(700,932)\" style=\"font-family:Verdana, Arial;font-size:200%\">Incoming Links</text>
      <line class=\"link-activeOutgoung\" x1=\"600\" y1=\"960\" x2=\"700\" y2=\"960\" style=\"stroke-width:12px;\" targetId=\"0\" sourceId=\"0\"/>
      <text transform=\"translate(700,972)\" style=\"font-family:Verdana, Arial;font-size:200%\">Outgoing Links</text>
    </g>
  </g>
  <g class=\"links\">")?;

    // This is used to reduce the calcualted coordiantes into a 950x950 box
    let abs_max = list_abs_max(&loc);
    eprintln!("Final well size: {:.3}", abs_max);
    let canvas_scale = 950.0 / abs_max;
    for edge in graph.edges.iter() {
        writeln!(&mut out, "    <line class=\"link\" targetId=\"{}\" sourceId=\"{}\" x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\"/>",
                 edge.src, edge.trg, 
                 trans(loc[edge.src * 2], canvas_scale), 
                 trans(loc[edge.src * 2 + 1], canvas_scale),
                 trans(loc[edge.trg * 2], canvas_scale), 
                 trans(loc[edge.trg * 2 + 1], canvas_scale))?;
    }
    writeln!(&mut out, "  </g>
  <g class=\"nodes\">")?;
    
    for i in 0..graph.n {
        writeln!(&mut out, "    <circle r=\"20\" cx=\"{}\" cy=\"{}\"/>",
                 trans(loc[i * 2], canvas_scale), 
                 trans(loc[i * 2 + 1], canvas_scale))?;
    }

    writeln!(&mut out, "  </g>
</svg>")
}

fn trans(x : f64, canvas_scale : f64) -> f64 {
    x * canvas_scale + 1000.0
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
