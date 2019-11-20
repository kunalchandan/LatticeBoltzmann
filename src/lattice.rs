extern crate nalgebra as na;

pub const SCALE_X: u32 = 8;
pub const SCALE_Y: u32 = 8;

pub const U_X: usize = 32;
pub const U_Y: usize = 32;
pub const X: u32 = U_X as u32;
pub const Y: u32 = U_Y as u32;

// Microscopic velocities
static E: [[f32; 2]; 9] = [[0.,0.],
                           [1.,0.], [ 0.,1.], [-1., 0.], [0.,-1.],
                           [1.,1.], [-1.,1.], [-1.,-1.], [1.,-1.]];
static W: [f32; 9] = [4./9.,
                      1./9.,  1./9.,  1./9.,  1./9.,
                      1./36., 1./36., 1./36., 1./36.];
// Microscopic velocities (using D2Q9 model)
// Yields 9 velocities per node

// Macroscopic fluid density = rho
// MFD = Density per node;
// Sum particle distribution

// Macroscopic velocity  = u
// Weighted average of microscopic velocities by distribution functions


#[derive(Copy, Clone)]
pub struct Node {
    micro_vel: [f32; 9],
    macro_vel: na::Vector2<f32>,

    micro_den: [f32; 9], // This is what we see as Fi or F in source [1]
    pub macro_den: f32,

    eq_dist: [f32; 9],
}

impl Node {
    pub fn calc_macro_vel(&mut self) {
        // Comes from Equation 4 of source [1] in README.md
        let mut sum = na::Vector2::new(0.,0.);
        for (i, _item) in self.micro_vel.iter().enumerate() {
            sum += self.micro_vel[i]*na::Vector2::new(E[i][0], E[i][1]);
        }
//        println!("MACRO_VEL::{}", sum);
        self.macro_vel = (1.0/self.macro_den)*sum;
    }

    pub fn calc_macro_den(&mut self) {
        // Comes from Equation 3 of source [1] in README.md
        let mut sum: f32 = 0.;
        for i in 0..self.micro_den.len() {
            sum += self.micro_den[i];
        }
        if sum.is_nan() {
            exit(-23);
        }
        self.macro_den = sum;
    }
}


pub struct Lattice {
    pub nodes: Vec<Vec<Node>>,
    c: f32, // Lattice Speed = dx/dt
    tau: f32
}

pub fn build_lattice() -> Lattice {
    let mut nodes: Vec<Vec<Node>> = vec![vec![Node{
                    micro_vel: [0.; 9],
                    macro_vel: na::Vector2::new(0.,0.),
                    micro_den: [1.; 9],
                    macro_den: 0.,
                    eq_dist: [0.; 9]
        }; U_X]; U_Y];
    for x in nodes.iter_mut() {
        for y in x.iter_mut() {
            y.calc_macro_den();
            y.calc_macro_vel();
        }
    }
    let mut boltz = Lattice {
        nodes,
        c: 2.0,
        tau: 1.0
    };
    return boltz;
}

impl Lattice {

    fn add_stuff(&mut self, cursor: [f64; 2]) {
        let c0 = cursor[0] as u32;
        let c1 = cursor[1] as u32;
        println!("{}, {}", (c0/SCALE_X) as usize, (c1/SCALE_Y) as usize);
        for den in 0..self.nodes[(c0/SCALE_X) as usize][(c1/SCALE_Y) as usize].micro_den.len() {
            self.nodes[(c0/SCALE_X) as usize][(c1/SCALE_Y) as usize].micro_den[den] += 1.0;
        }
        for vel in 0..self.nodes[(c0/SCALE_X) as usize][(c1/SCALE_Y) as usize].micro_vel.len() {
            self.nodes[(c0/SCALE_X) as usize][(c1/SCALE_Y) as usize].micro_vel[vel] += 10. * unsafe { (vel as f32).sin() };
        }
    }

    fn calc_macros(&mut self) {
        // Part 0:
        // Recalculate macro fluid density for each node
        // Recalculate macro fluid velocity for each node
        for row in self.nodes.iter_mut() {
            for cell in row.iter_mut() {
                cell.calc_macro_den();
                cell.calc_macro_vel();
            }
        }
    }

    fn streaming(&mut self) {
        // Part 1:
        // Streaming -> push all microscopic velocities to other nodes.
        // Will need an empty copy of microscopic velocities to push into
        // Fi -> Fi* in direction of micro-vel

        // Vectors for streaming are like this::
        /*
         6 | 2 | 5
        ---+---+---
         3 | 0 | 1
        ---+---+---
         7 | 4 | 8
        */

        // My streaming Algorithm;
        // Note:: I think I might've been inspired by source [3], but I didn't read it lol
        // We will solve this inplace.
        // 1st pass swap closest incoming and outgoing micro velocity

        // Side Note:: Cannot use std::mem::swap because of the borrow checker, and I don't want to
        // deal with slices and all this other stuff to guarantee safety
        // while doing unsafe stuff with pointers
        for x in 0..self.nodes.len() {
            for y in 0..self.nodes[x].len() {
                // Swap for each of the 9 directions.
                if x > 0 {
                    let _s_ = self.nodes[ x ][y].micro_vel[3];
                    self.nodes[ x ][y].micro_vel[3] = self.nodes[x-1][y].micro_vel[1];
                    self.nodes[x-1][y].micro_vel[1] = _s_;
                    if y > 0 {
                        let _s_ = self.nodes[ x ][ y ].micro_vel[6];
                        self.nodes[ x ][ y ].micro_vel[6] = self.nodes[x-1][y-1].micro_vel[8];
                        self.nodes[x-1][y-1].micro_vel[8] = _s_;
                    }
                    if y < U_Y - 1 {
                        let _s_ = self.nodes[ x ][ y ].micro_vel[7];
                        self.nodes[ x ][ y ].micro_vel[7] = self.nodes[x-1][y+1].micro_vel[5];
                        self.nodes[x-1][y+1].micro_vel[5] = _s_;
                    }
                }
                if x < U_X - 1 {
                    let _s_ = self.nodes[ x ][y].micro_vel[1];
                    self.nodes[ x ][y].micro_vel[1] = self.nodes[x+1][y].micro_vel[3];
                    self.nodes[x+1][y].micro_vel[3] = _s_;
                    if y > 0 {
                        let _s_ = self.nodes[ x ][ y ].micro_vel[5];
                        self.nodes[ x ][ y ].micro_vel[5] = self.nodes[x+1][y-1].micro_vel[7];
                        self.nodes[x+1][y-1].micro_vel[7] = _s_;
                    }
                    if y < U_Y - 1 {
                        let _s_ = self.nodes[ x ][ y ].micro_vel[8];
                        self.nodes[ x ][ y ].micro_vel[8] = self.nodes[x+1][y+1].micro_vel[6];
                        self.nodes[x+1][y+1].micro_vel[6] = _s_;
                    }
                }
                if y > 0 {
                    let _s_ = self.nodes[x][ y ].micro_vel[2];
                    self.nodes[x][ y ].micro_vel[2] = self.nodes[x][y-1].micro_vel[4];
                    self.nodes[x][y-1].micro_vel[4] = _s_;
                }
                if y < U_Y - 1 {
                    let _s_ = self.nodes[x][ y ].micro_vel[4];
                    self.nodes[x][ y ].micro_vel[4] = self.nodes[x][y+1].micro_vel[2];
                    self.nodes[x][y+1].micro_vel[2] = _s_;
                }

                // TODO:: Resolve Boundary conditions
            }
        }

        // 2nd pass swap the now flipped micro velocities in each cell
        for x in 0..self.nodes.len() {
            for y in 0..self.nodes[x].len() {
                let _s_ = self.nodes[x][y].micro_vel[1];
                self.nodes[x][y].micro_vel[1] = self.nodes[x][y].micro_vel[3];
                self.nodes[x][y].micro_vel[3] = _s_;

                let _s_ = self.nodes[x][y].micro_vel[2];
                self.nodes[x][y].micro_vel[2] = self.nodes[x][y].micro_vel[4];
                self.nodes[x][y].micro_vel[4] = _s_;

                let _s_ = self.nodes[x][y].micro_vel[5];
                self.nodes[x][y].micro_vel[5] = self.nodes[x][y].micro_vel[7];
                self.nodes[x][y].micro_vel[7] = _s_;

                let _s_ = self.nodes[x][y].micro_vel[6];
                self.nodes[x][y].micro_vel[6] = self.nodes[x][y].micro_vel[8];
                self.nodes[x][y].micro_vel[8] = _s_;
            }
        }
    }

    fn calc_eq_dist(&mut self) {
        // Calculate the Equilibrium Distribution.
        // Using BKG algorithm/equation from source [1] equation 6
        // Feq[i] = macro_den*(W[i] + S[i](macro_vel)
        let c = self.c;
        for x in 0..self.nodes.len() {
            for y in 0..self.nodes[x].len() {

                let rho = self.nodes[x][y].macro_den;
                let u = self.nodes[x][y].macro_vel;

                for i in 0..self.nodes[x][y].eq_dist.len() {
                    let wi = W[i];
                    let ei: na::Vector2<f32> = na::Vector2::new(E[i][0], E[i][1]);
                    self.nodes[x][y].eq_dist[i] = rho * wi * (1. + (
                        (3. * ei.dot(&u) / c) +
                        (9. * ei.dot(&u).powi(2) / (2. * c.powi(2))) -
                        (3. * u.dot(&u) / (2. * c.powi(2)))));
                }
            }
        }
    }

    fn collision(&mut self) {
        // Part 2:
        // Collision ->
        // Fi = Fi*  - 1/T ( Fi* - Fi eq)
        for x in 0..self.nodes.len() {
            for y in 0..self.nodes[x].len() {
                for i in 0..self.nodes[x][y].micro_den.len() {
                    let fi = self.nodes[x][y].micro_den[i];
                    let feq = self.nodes[x][y].eq_dist[i];
                    self.nodes[x][y].micro_den[i] = fi - ((fi - feq) / self.tau);
                }
            }
        }
    }

    pub fn update(&mut self, cursor: [f64; 2]) {
        self.add_stuff(cursor);
        self.calc_macros();
        self.streaming();
        self.calc_eq_dist();
        self.collision();
//        for x in 0..U_X {
//            for y in  0..U_Y {
//                print!("{}, ", self.nodes[x][y].macro_den);
//            }
//            println!();
//        }
    }
}