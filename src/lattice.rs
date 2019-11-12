
const U_X: usize = 512;
const U_Y: usize = 128;
const X: u32 = U_X as u32;
const Y: u32 = U_Y as u32;

// Microscopic velocities
static E: [[i8; 2]; 9] = [[0,0],
                          [1,0], [ 0,1], [-1, 0], [0,-1],
                          [1,1], [-1,1], [-1,-1], [1,-1]];
// Microscopic velocities (using D2Q9 model)
// Yields 9 velocities per node

// Macroscopic fluid density = rho
// MFD = Density per node;
// Sum particle distribution

// Macroscopic velocity  = u
// Weighted average of microscopic velocities by distribution functions

// Streaming -> push all microscopic velocities to other nodes.
// Will need an empty copy of microscopic velocities to push into
// Fi -> Fi* in direction of micro-vel

// Collision ->
// Fi = Fi*  - 1/T ( Fi* - Fi eq)

pub struct Node {
    micro_vel: [f32; 9],
    macro_vel: [f32; 2],

    micro_den: [f32; 9],
    macro_den: [f32; 2]
}

impl Node {
    pub fn calc_macro_vel(&mut self) {
        self.macro_vel = [0.7, 0.9];
    }
}


pub struct Lattice {
    nodes: [[Node; U_X]; U_Y],
}

