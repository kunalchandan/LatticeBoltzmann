extern crate nalgebra as na;

use std::process::exit;
use crate::lattice::node::Node;
use crate::lattice::consts::*;

mod node;
mod funcs;
mod consts;


// Microscopic velocities (using D2Q9 model)
// Yields 9 velocities per node

// Macroscopic fluid density = rho
// MFD = Density per node;
// Sum particle distribution

// Macroscopic velocity  = u
// Weighted average of microscopic velocities by distribution functions

pub struct Lattice {
    pub nodes: Vec<Vec<Node>>,
    c: f32, // Lattice Speed = dx/dt
    tau: f32
}

pub fn build_lattice() -> Lattice {
    let mut nodes: Vec<Vec<Node>> = vec![vec![Node{
                    micro_vel: [0.5; 9],
                    macro_vel: na::Vector2::new(0.,0.),
                    micro_den: [0.5; 9],
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
        c: C,
        tau: T
    };
    return boltz;
}

impl Lattice {
    pub fn update(&mut self, cursor: [f64; 2]) {
        self.add_stuff(cursor);
        self.clamp_values();
        self.calc_macros();
        let c0 = cursor[0] as u32;
        let c1 = cursor[1] as u32;
        self.streaming();
        self.calc_eq_dist();
        self.collision();
    }
}