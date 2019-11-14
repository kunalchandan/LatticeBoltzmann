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

// Streaming -> push all microscopic velocities to other nodes.
// Will need an empty copy of microscopic velocities to push into
// Fi -> Fi* in direction of micro-vel

// Collision ->
// Fi = Fi*  - 1/T ( Fi* - Fi eq)

#[derive(Copy, Clone)]
pub struct Node {
    micro_vel: [f32; 9],
    macro_vel: na::Vector2<f32>,

    micro_den: [f32; 9],
    macro_den: na::Vector2<f32>,
}

impl Node {
    pub fn calc_macro_vel(&mut self) {

    }

    pub fn calc_macro_den(&mut self) {
        let mut sum = na::Vector2::new(0.,0.);
        for (i, item) in self.micro_den.iter().enumerate() {
            sum += self.micro_den[i]*na::Vector2::new(E[i][0], E[i][1]);
        }
        println!("{}", sum);
        println!("YU");
        self.macro_den = sum;
    }
}


pub struct Lattice {
    pub nodes: Box<[[Node; U_X]; U_Y]>,
}

pub fn build_lattice() -> Lattice {
    let mut rng = rand::thread_rng();

    let mut nodes: Box<[[Node; U_X]; U_Y]> = unsafe { Box::from_raw(
        vec![[Node{
                    micro_vel: [rng.gen(); 9],
                    macro_vel: na::Vector2::new(0.,0.),
                    micro_den: [rng.gen(); 9],
                    macro_den: na::Vector2::new(0.,0.),
                }; U_X]; U_Y].into_boxed_slice().as_mut_ptr() as * mut _)
    };
    let mut boltz = Lattice {
        nodes
    };
    return boltz;
}
