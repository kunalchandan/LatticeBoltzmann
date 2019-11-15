extern crate nalgebra as na;
extern crate rand;

use self::rand::Rng;

const U_X: usize = 64;
const U_Y: usize = 64;
const X: u32 = U_X as u32;
const Y: u32 = U_Y as u32;

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

    micro_den: [f32; 9],
    macro_den: f32,

    eq_dist: [f32; 9],
}

impl Node {
    pub fn calc_macro_vel(&mut self) {
        // Comes from Equation 4 of source [1] in README.md
        let mut sum = na::Vector2::new(0.,0.);
        for (i, item) in self.micro_vel.iter().enumerate() {
            sum += self.micro_vel[i]*na::Vector2::new(E[i][0], E[i][1]);
        }
        println!("MACRO_VEL::{}", sum);
        self.macro_vel = (1.0/self.macro_den)*sum;
    }

    pub fn calc_macro_den(&mut self) {
        // Comes from Equation 3 of source [1] in README.md
        let mut sum = 0.;
        for (i, item) in self.micro_den.iter().enumerate() {
            sum += self.micro_den[i];
        }
        println!("MACRO_DEN::{}", sum);
        self.macro_den = sum;
    }
}


pub struct Lattice {
    pub nodes: Box<[[Node; U_X]; U_Y]>,
    c: f32, // Lattice Speed = dx/dt
}

pub fn build_lattice() -> Lattice {
    let mut rng = rand::thread_rng();

    let mut nodes: Box<[[Node; U_X]; U_Y]> = unsafe { Box::from_raw(
        vec![[Node{
                    micro_vel: [rng.gen(); 9],
                    macro_vel: na::Vector2::new(0.,0.),
                    micro_den: [rng.gen(); 9],
                    macro_den: na::Vector2::new(0.,0.),
                    eq_dist: [0.; 9]
        }; U_X]; U_Y].into_boxed_slice().as_mut_ptr() as * mut _)
    };
    let mut boltz = Lattice {
        nodes,
        c: 2.0
    };
    return boltz;
}

impl Lattice {

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
        for (x, _x) in self.nodes.iter_mut().enumerate() {
            for (y, _y) in _x.iter_mut().enumerate() {
                // Swap for each of the 9 directions.
                if x > 0 {
                    std::mem::swap(self.nodes[ x ][y].micro_vel[3],
                                   self.nodes[x-1][y].micro_vel[1]);
                    if y > 0 {
                        std::mem::swap(self.nodes[ x ][y].micro_vel[6],
                                       self.nodes[x-1][y-1].micro_vel[8]);
                    }
                    if y < U_Y - 1 {
                        std::mem::swap(self.nodes[ x ][ y ].micro_vel[7],
                                       self.nodes[x-1][y+1].micro_vel[5]);
                    }
                }
                if x < U_X - 1 {
                    std::mem::swap(self.nodes[ x ][y].micro_vel[1],
                                   self.nodes[x+1][y].micro_vel[3]);
                    if y > 0 {
                        std::mem::swap(self.nodes[ x ][ y ].micro_vel[8],
                                       self.nodes[x+1][y+1].micro_vel[6]);
                    }
                    if y < U_Y - 1 {
                        std::mem::swap(self.nodes[ x ][ y ].micro_vel[5],
                                       self.nodes[x+1][y-1].micro_vel[7]);
                    }
                }
                if y > 0 {
                    std::mem::swap(self.nodes[x][ y ].micro_vel[2],
                                   self.nodes[x][y-1].micro_vel[4]);
                }
                if y < U_Y - 1 {
                    std::mem::swap(self.nodes[x][ y ].micro_vel[4],
                                   self.nodes[x][y+1].micro_vel[2]);
                }
            }
        }

        // 2nd pass swap the now flipped micro velocities in each cell
        for (x, _x) in self.nodes.iter_mut().enumerate() {
            for (y, _y) in _x.iter_mut().enumerate() {
                std::mem::swap(self.nodes[x][y].micro_vel[1],
                               self.nodes[x][y].micro_vel[3]);
                std::mem::swap(self.nodes[x][y].micro_vel[2],
                               self.nodes[x][y].micro_vel[4]);
                std::mem::swap(self.nodes[x][y].micro_vel[5],
                               self.nodes[x][y].micro_vel[7]);
                std::mem::swap(self.nodes[x][y].micro_vel[6],
                               self.nodes[x][y].micro_vel[8]);
            }
        }
    }

    fn calc_eq_dist(&mut self) {
        // Calculate the Equilibrium Distribution.
        // Using BKG algorithm/equation from source [1] equation 6
        // Feq[i] = macro_den*(W[i] + S[i](macro_vel)
        for (x, _x) in self.nodes.iter_mut().enumerate() {
            for (y, _y) in _x.iter_mut().enumerate() {
                for (i, _i) in _y.eq_dist.iter_mut().enumerate() {
                    let rho = self.nodes[x][y].macro_den;
                    let u = self.nodes[x][y].macro_vel;
                    let w = W[i];
                    let ei: na::Vector2<f32> = na::Vector2::new(E[i][0], E[i][1]);
                    self.nodes[x][y].eq_dist[i] =  rho * w * ( 1 + ((3*ei.dot(u))))

                }
            }
        }
    }

    pub fn update(&mut self) {
        // TODO:: Do the 2 steps of the method
        self.streaming();
        self.calc_macros();
        self.calc_eq_dist();
        self.collision();


        for (x, _x) in self.nodes.iter_mut().enumerate() {
            for (y, _y) in _x.iter_mut().enumerate() {
                self.nodes[x][y].calc_macro_den();
                self.nodes[x][y].calc_macro_vel();
            }
        }
        // Part 2:
        // Collision ->
        // Fi = Fi*  - 1/T ( Fi* - Fi eq)
    }
}