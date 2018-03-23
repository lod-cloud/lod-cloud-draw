//! Layout algorithm to set the initial conditions of the graph.
//! This attempts to build a tree in the following method
//! 1. Find the most connected central node.
//! 2. Fan all nodes connected to the centre in a 360° arc.
//! 3. Recursively fan each other node out in a 180° arc.
use graph::Graph;
use std::f64::consts::PI;
use std::collections::HashSet;

/// Builds a tree layout from a graph, where `radius` is the radius of the 
/// span. The return value is the list of [x0,y0,...,xn,yn]
///
/// # Examples
///
/// ```
/// let mut g = Graph::new();
/// let v1 = g.add_vertex("v1");
/// let v2 = g.add_vertex("v2");
/// let v3 = g.add_vertex("v3");
/// let v4 = g.add_vertex("v4");
/// let v5 = g.add_vertex("v5");
/// g.edges.push(Edge::new(v1, v2));
/// g.edges.push(Edge::new(v1, v3));
/// g.edges.push(Edge::new(v1, v4));
/// g.edges.push(Edge::new(v4, v5));
///
/// let result = build_tree(&g, 10.0);
/// ```
pub fn build_tree(graph : &Graph, radius : f64) -> Vec<f64> {
    if graph.n == 0 {
        panic!("Graph is empty");
    }

    let mut nodes = HashSet::new();
    nodes.extend(0..graph.n);

    let mut edges = Vec::new();
    edges.resize(graph.n, Vec::new());

    for edge in graph.edges.iter() {
        edges[edge.src].push(edge.trg);
        edges[edge.trg].push(edge.src);
    }

    let mut v0 = 0;
    let mut max_edges = 0;
    for i in 0..graph.n {
        if edges[i].len() > max_edges {
            v0 = i;
            max_edges = edges[i].len();
        }
    }

    nodes.remove(&v0);

    let mut loc = Vec::new();
    loc.resize(graph.n * 2, 0.0f64);
    
    calculate_loc(&mut loc, v0, &mut nodes, &edges, graph, radius);

    push_nearby(loc, radius)
}

fn push_nearby(mut loc : Vec<f64>, radius : f64) -> Vec<f64> {
    for i in 0..(loc.len() / 2) {
        for j in 0..(loc.len() / 2) {
            if i < j {
                let x = loc[i * 2] - loc[j * 2];
                let y = loc[i * 2 + 1] - loc[j * 2 + 1];
                let d = (x * x + y * y).sqrt();
                if d < radius * 0.01 {
                    loc[i * 2] += radius * 0.01 * (i as f64).cos();
                    loc[i * 2 + 1] += radius * 0.01 * (i as f64).sin();
                    loc[j * 2] += radius * 0.01 * (j as f64).cos();
                    loc[j * 2 + 1] += radius * 0.01 * (j as f64).sin();
                }
            }
        }
    }
    loc
}

// Convert an (x,y) coordinate to an angle where (1,0) => 0
fn to_angle(x : f64, y : f64) -> f64 {
    if x > 0.0 {
        if y > 0.0 {
            (y/x).atan()
        } else if y < 0.0 {
            2.0 * PI + (y/x).atan()
        } else {
            0.0
        }
    } else if x < 0.0 {
        if y > 0.0 {
            PI + (y/x).atan()
        } else if y < 0.0 {
            PI + (y/x).atan()
        } else {
            PI
        }
    } else {
        if y > 0.0 {
            PI / 2.0
        } else if y < 0.0 {
            3.0 * PI / 2.0
        } else {
            0.0
        }
    }
}




fn calculate_loc(loc : &mut Vec<f64>, parent : usize, nodes : &mut HashSet<usize>,
                 edges : &Vec<Vec<usize>>, graph : &Graph, radius : f64) {

    let children : Vec<&usize> = edges[parent].iter().
        filter(|x| nodes.contains(x)).collect();

    let astep = if loc[parent * 2] == 0.0 && loc[parent * 2 + 1] == 0.0 {
        2.0 * PI / (children.len() as f64)
    } else {
        PI / ((children.len() + 1) as f64)
    };

    let ainitial = to_angle(loc[parent * 2], loc[parent * 2 + 1]) - PI / 2.0;

    for child in children.iter() {
        nodes.remove(child);
    }

    let mut a = 1.0;
    for &child in children.iter() {
        loc[child * 2] = loc[parent * 2] + radius * (ainitial + a * astep).cos();
        loc[child * 2 + 1] = loc[parent * 2 + 1] + radius * (ainitial + a * astep).sin();

        calculate_loc(loc, *child, nodes, edges, graph, radius);
        a += 1.0;
    } 

}

#[cfg(test)]
mod tests {
    use graph::{Graph, Edge};
    use tree::{build_tree, to_angle, push_nearby};
    use std::f64::consts::PI;

    #[test]
    #[allow(non_snake_case)]
    fn test_angle() {
        let Z = f64::sqrt(3.0) / 2.0;
        assert!((to_angle(1.0,0.0) - 0.0) < 1e-4);
        assert!((to_angle(0.0,1.0) - PI / 2.0) < 1e-4);
        assert!((to_angle(-1.0,0.0) - PI) < 1e-4);
        assert!((to_angle(0.0,-1.0) - 3.0 * PI / 2.0) < 1e-4);
        assert!((to_angle(1.0,1.0) - PI / 4.0) < 1e-4); 
        assert!((to_angle(-1.0,1.0) - 3.0 * PI / 4.0) < 1e-4); 
        assert!((to_angle(-1.0,-1.0) - 5.0 * PI / 4.0) < 1e-4); 
        assert!((to_angle(1.0,-1.0) - 7.0 * PI / 4.0) < 1e-4); 
        assert!((to_angle(Z, 0.5) - PI / 6.0) < 1e-4);
        assert!((to_angle(0.5, Z) - 2.0 * PI / 6.0) < 1e-4);
        assert!((to_angle(-0.5, Z) - 4.0 * PI / 6.0) < 1e-4);
        assert!((to_angle(-Z, 0.5) - 5.0 * PI / 6.0) < 1e-4);
        assert!((to_angle(-Z, -0.5) - 7.0 * PI / 6.0) < 1e-4);
        assert!((to_angle(-0.5, -Z) - 8.0 * PI / 6.0) < 1e-4);
        assert!((to_angle(0.5, -Z) - 10.0 * PI / 6.0) < 1e-4);
        assert!((to_angle(Z, -0.5) - 11.0 * PI / 6.0) < 1e-4);
    }

    #[test]
    fn test_push_nearby() {
        let mut v = Vec::new();
        v.resize(10,0.0f64);
        v = push_nearby(v, 1.0);
        println!("{:?}", v);
        for i in 0..5 {
            for j in 0..5 {
                if i != j {
                    let x = v[i * 2] - v[j * 2];
                    let y = v[i * 2 + 1] - v[j * 2 + 1];

                    assert!(x*x + y*y > 0.0);
                }
            }
        }
    }


    #[test]
    fn test_tree() {
        let mut g = Graph::new();
        let v1 = g.add_vertex("v1");
        let v2 = g.add_vertex("v2");
        let v3 = g.add_vertex("v3");
        let v4 = g.add_vertex("v4");
        let v5 = g.add_vertex("v5");
        g.edges.push(Edge::new(v1, v2));
        g.edges.push(Edge::new(v1, v3));
        g.edges.push(Edge::new(v1, v4));
        g.edges.push(Edge::new(v4, v5));

        let result = build_tree(&g, 10.0);

        let exp = [0.0,0.0,
                   10.0 * (PI / 6.0).cos(), 10.0 * (PI/6.0).sin(),
                   -10.0 * (PI / 6.0).cos(), 10.0 * (PI/6.0).sin(),
                   0.0,-10.0,
                   0.0,-20.0
        ];

        for i in 0..exp.len() {
            println!("{}: {:.3} == {:.3}",i, exp[i], result[i]);
            assert!((exp[i] - result[i]).abs() < 1e-4);
        } 
    }
}
