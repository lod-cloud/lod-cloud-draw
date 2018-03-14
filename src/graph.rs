#[derive(Debug,PartialEq,Clone)]
pub struct Graph {
    pub n: usize,
    pub edges: Vec<Edge>
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            n: 0,
            edges : Vec::new()
        }
    }

    pub fn add_vertex(&mut self) -> usize {
        self.n += 1;
        (self.n - 1)
    }

    pub fn spiral(&self) -> Vec<f64> {
        let mut v = Vec::new();
        for i in 0..self.n {
            let i_f = i as f64;
            v.push(i_f * f64::cos(i_f));
            v.push(i_f * f64::sin(i_f));
        }
        v
    }

    //fn print_graph(&self, x : &Vec<f64>) {
    //    println!("Graph of {} vertices", self.n);
    //    for id in 0..self.n {
    //        println!("{}: ({:.3}, {:.3})", id, x[id * 2], x[id * 2 + 1]);
    //    }
    //}
    //

    pub fn cost2(&self, loc : &Vec<f64>, spring : f64, repulse : f64, smin : f64,
                 centre : f64, roundness : f64, canvas_size : f64,
                 n_blocks : usize) -> f64 {
        let mut cost = 0.0;

        for edge in self.edges.iter() {
            let x = loc[edge.src * 2] - loc[edge.trg * 2];
            let y = loc[edge.src * 2 + 1] - loc[edge.trg * 2 + 1];
            let d = (x * x + y * y).sqrt();
            cost += spring * d;
        }

        if n_blocks > 1 {
            let blocking = Blocking::create(loc, n_blocks);

            for v1 in 0..self.n {
                for &(v2_id, v2_x, v2_y) in blocking.nearby(loc[v1 * 2], loc[v1 * 2 + 1]).iter() {
                    if v1 != v2_id {
                        let x = loc[v1 * 2] - v2_x;
                        let y = loc[v1 * 2 + 1] - v2_y;
                        // Sigmoid repulsion factor
                        cost += repulse / (1.0 + ((x * x + y * y).sqrt() - smin).exp());
                    }
                }
                // Canvas bound
                let d = (loc[v1 * 2] * loc[v1 * 2] + 
                         loc[v1 * 2 + 1] * loc[v1 * 2 + 1]).sqrt();
                cost += centre * (d / canvas_size).powf(roundness);
            }
        } else {
            for v1 in 0..self.n {
                for v2 in 0..self.n {
                    if v1 != v2 {
                        let x = loc[v1 * 2] - loc[v2 * 2];
                        let y = loc[v1 * 2 + 1] - loc[v2 * 2 + 1];
                        cost += repulse / (1.0 + (smin - (x * x + y * y).sqrt()).exp());
                    }
                }
                // Centre attraction
                let d = (loc[v1 * 2] * loc[v1 * 2] + 
                         loc[v1 * 2 + 1] * loc[v1 * 2 + 1]).sqrt();
                cost += centre * (d / canvas_size).powf(roundness);
            }
        }
        cost
    }

    pub fn gradient2(&self, loc : &Vec<f64>, spring : f64, repulse : f64, smin : f64,
                 centre : f64, roundness : f64, canvas_size : f64,
                 n_blocks : usize) -> Vec<f64> {
        let mut gradient = Vec::new();
        gradient.resize(self.n * 2, 0.0f64);
        // Spring cost ||vi - vj||^2
        for edge in self.edges.iter() {
            let x = loc[edge.src * 2] - loc[edge.trg * 2];
            let y = loc[edge.src * 2 + 1] - loc[edge.trg * 2 + 1];
            let d = (x * x + y * y).sqrt();

            if d != 0.0 {
                gradient[edge.src * 2] += spring * x / d;
                gradient[edge.src * 2 + 1] += spring * y / d;
                gradient[edge.trg * 2] -= spring * x / d;
                gradient[edge.trg * 2 + 1] -= spring * y / d;
            }
        }

        if n_blocks > 1 {
            let blocking = Blocking::create(loc, n_blocks);
            for v1 in 0..self.n {
                for &(v2_id, v2_x, v2_y) in blocking.nearby(loc[v1 * 2], loc[v1 * 2 + 1]).iter() {
                    // Repulsion 1/||vi - vj||
                    if v1 != v2_id {
                        let x = loc[v1 * 2] - v2_x;
                        let y = loc[v1 * 2 + 1] - v2_y;
                        let d = (x*x + y*y).sqrt();
                        let s = sigma(smin - d);
                        gradient[v1 * 2] += repulse * 2.0 * s * (1.0 - s) / d * x;
                        gradient[v1 * 2 + 1] +=  repulse * 2.0 * s * (1.0 - s) / d * y;
                    }
                }
                // Centre attraction
                let d = (loc[v1 * 2] * loc[v1 * 2] + 
                         loc[v1 * 2 + 1] * loc[v1 * 2 + 1]).sqrt();
                gradient[v1 * 2] += centre * 
                    canvas_size.powf(-roundness) *
                    roundness * loc[v1 * 2] *
                    d.powf(roundness - 2.0);
                gradient[v1 * 2 + 1] += centre * 
                    canvas_size.powf(-roundness) *
                    roundness * loc[v1 * 2 + 1] *
                    d.powf(roundness - 2.0);
             }
        } else {
             for v1 in 0..self.n {
                for v2 in 0..self.n {
                    // Repulsion 1/||vi - vj||
                    if v1 != v2 {
                        let x = loc[v1 * 2] - loc[v2 * 2];
                        let y = loc[v1 * 2 + 1] - loc[v2 * 2 + 1];
                        let d = (x*x + y*y).sqrt();
                        let s = sigma(smin - d);
                        gradient[v1 * 2] += repulse * 2.0 * s * (1.0 - s) / d * x;
                        gradient[v1 * 2 + 1] +=  repulse * 2.0 * s * (1.0 - s) / d * y;
                    }
                }
                // Centre attraction
                let d = (loc[v1 * 2] * loc[v1 * 2] + 
                         loc[v1 * 2 + 1] * loc[v1 * 2 + 1]).sqrt();
                gradient[v1 * 2] += centre * 
                    canvas_size.powf(-roundness) *
                    roundness * loc[v1 * 2] *
                    d.powf(roundness - 2.0);
                gradient[v1 * 2 + 1] += centre * 
                    canvas_size.powf(-roundness) *
                    roundness * loc[v1 * 2 + 1] *
                    d.powf(roundness - 2.0);
             }
        }
        gradient
    }

    pub fn cost(&self, loc : &Vec<f64>, spring : f64, repulse : f64, smin : f64,
                centre : f64, n_blocks : usize) -> f64 {
        let mut cost = 0.0;

        for edge in self.edges.iter() {
            let x = loc[edge.src * 2] - loc[edge.trg * 2];
            let y = loc[edge.src * 2 + 1] - loc[edge.trg * 2 + 1];
            let d = x * x + y * y;
            if d > smin {
                cost += spring * (d - smin);
            }
        }

        if n_blocks > 1 {
            let blocking = Blocking::create(loc, n_blocks);

            for v1 in 0..self.n {
                for &(v2_id, v2_x, v2_y) in blocking.nearby(loc[v1 * 2], loc[v1 * 2 + 1]).iter() {
                    if v1 != v2_id {
                        let x = loc[v1 * 2] - v2_x;
                        let y = loc[v1 * 2 + 1] - v2_y;
                        cost += repulse * 1.0 / f64::sqrt(x * x + y * y);
                    }
                }
                // Centre attraction
                cost += centre * loc[v1 * 2] * loc[v1 * 2];
                cost += centre * loc[v1 * 2 + 1] * loc[v1 * 2 + 1];
            }
        } else {
            for v1 in 0..self.n {
                for v2 in 0..self.n {
                    if v1 != v2 {
                        let x = loc[v1 * 2] - loc[v2 * 2];
                        let y = loc[v1 * 2 + 1] - loc[v2 * 2 + 1];
                        cost += repulse * 1.0 / f64::sqrt(x * x + y * y);
                    }
                }
                // Centre attraction
                cost += centre * loc[v1 * 2] * loc[v1 * 2];
                cost += centre * loc[v1 * 2 + 1] * loc[v1 * 2 + 1];
            }
        }
        cost
    }


    pub fn gradient(&self, loc : &Vec<f64>, spring : f64, repulse : f64, 
                    smin : f64, centre : f64, n_blocks : usize) -> Vec<f64> {
        let mut gradient = Vec::new();
        gradient.resize(self.n * 2, 0.0f64);
        // Spring cost ||vi - vj||^2
        for edge in self.edges.iter() {
            let x = loc[edge.src * 2] - loc[edge.trg * 2];
            let y = loc[edge.src * 2 + 1] - loc[edge.trg * 2 + 1];
            let d = x * x + y * y;

            if d > smin {
                gradient[edge.src * 2] += spring * 2.0 * x;
                gradient[edge.src * 2 + 1] += spring * 2.0 * y;
                gradient[edge.trg * 2] -= spring * 2.0 * x;
                gradient[edge.trg * 2 + 1] -= spring * 2.0 * y;
            }

        }

        if n_blocks > 1 {
            let blocking = Blocking::create(loc, n_blocks);
            for v1 in 0..self.n {
                for &(v2_id, v2_x, v2_y) in blocking.nearby(loc[v1 * 2], loc[v1 * 2 + 1]).iter() {
                    // Repulsion 1/||vi - vj||
                    if v1 != v2_id {
                        let x = loc[v1 * 2] - v2_x;
                        let y = loc[v1 * 2 + 1] - v2_y;
                        let m = f64::sqrt(x*x + y*y);
                        gradient[v1 * 2] -= repulse * 2.0 * x / m / m / m;
                        gradient[v1 * 2 + 1] -=  repulse * 2.0 * y / m / m / m;
                    }
                }
                // Centre attraction
                gradient[v1 * 2] += centre * 2.0 * loc[v1 * 2];
                gradient[v1 * 2 + 1] += centre * 2.0 * loc[v1 * 2 + 1];
             }
        } else {
             for v1 in 0..self.n {
                for v2 in 0..self.n {
                    // Repulsion 1/||vi - vj||
                    if v1 != v2 {
                        let x = loc[v1 * 2] - loc[v2 * 2];
                        let y = loc[v1 * 2 + 1] - loc[v2 * 2 + 1];
                        let m = f64::sqrt(x*x + y*y);
                        gradient[v1 * 2] -= repulse * 2.0 * x / m / m / m;
                        gradient[v1 * 2 + 1] -=  repulse * 2.0 * y / m / m / m;
                    }
                }
                // Centre attraction
                gradient[v1 * 2] += centre * 2.0 * loc[v1 * 2];
                gradient[v1 * 2 + 1] += centre * 2.0 * loc[v1 * 2 + 1];
            }
        }
        gradient
    }
}

#[derive(Debug,PartialEq,Clone)]
pub struct Edge {
    pub src : usize,
    pub trg : usize
}

impl Edge {
    pub fn new(from : usize, to  : usize) -> Edge {
        Edge {
            src: from,
            trg: to
        }
    }
}

struct Blocking {
    blocks : Vec<Vec<Vec<(usize,f64,f64)>>>,
    block_size : f64,
    max : f64,
    n_blocks : usize
}

impl Blocking {
    fn create(xs : &Vec<f64>, n_blocks : usize) -> Blocking {
        let mut blocks = Vec::new();
        blocks.resize(n_blocks, Vec::new());
        for i in 0..n_blocks {
            blocks[i].resize(n_blocks, Vec::new());
        }

        let mut max = 0.0;
        for x in xs {
            if x.is_finite() && x.abs() > max {
                max = x.abs();
            }
        }
        max *= 1.01; // To ensure that no value is exactly on the block boundary
        let block_size = max * 2.0 / (n_blocks as f64);

        for i in 0..(xs.len() / 2) {
            let x = ((xs[i * 2] + max) / block_size).floor() as usize;
            let y = ((xs[i * 2 + 1] + max) / block_size).floor() as usize;

            blocks[x][y].push((i, xs[i * 2], xs[i * 2 + 1]));
        }
        Blocking {
            blocks: blocks,
            block_size: block_size,
            max : max,
            n_blocks
        }
    }

    fn nearby<'a>(&'a self, x : f64, y : f64) -> Vec<(usize, f64, f64)> {
        let x_id = ((x + self.max) / self.block_size).floor() as usize;
        let y_id = ((y + self.max) / self.block_size).floor() as usize;

        let mut elems = self.blocks[x_id][y_id].clone();
        if x_id > 0 {
            if y_id > 0 {
                elems.extend(self.blocks[x_id - 1][y_id - 1].iter());
            }
            if y_id < self.n_blocks - 1 {
                elems.extend(self.blocks[x_id - 1][y_id + 1].iter());
            }
            elems.extend(self.blocks[x_id - 1][y_id].iter());
        }
        if x_id < self.n_blocks - 1 {
            if y_id > 0 {
                elems.extend(self.blocks[x_id + 1][y_id - 1].iter());
            }
            if y_id < self.n_blocks - 1 {
                elems.extend(self.blocks[x_id + 1][y_id + 1].iter());
            }
            elems.extend(self.blocks[x_id + 1][y_id].iter());
        }
        if y_id > 0 {
            elems.extend(self.blocks[x_id][y_id - 1].iter());
        }
        if y_id < self.n_blocks - 1 {
            elems.extend(self.blocks[x_id][y_id + 1].iter());
        }
 
        elems
    }
}

fn sigma(x : f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}


