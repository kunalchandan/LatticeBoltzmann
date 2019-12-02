use crate::lattice::consts::E;

#[derive(Copy, Clone)]
pub struct Node {
    pub micro_vel: [f32; 9],
    pub macro_vel: na::Vector2<f32>,

    pub micro_den: [f32; 9], // This is what we see as Fi or F in source [1]
    pub macro_den: f32,

    pub eq_dist: [f32; 9],
}

impl Node {
    pub fn calc_macro_vel(&mut self) {
        // Comes from Equation 4 of source [1] in README.md
        let mut sum = na::Vector2::new(0.,0.);
        for (i, _item) in self.micro_vel.iter().enumerate() {
            sum += self.micro_vel[i]*na::Vector2::new(E[i][0], E[i][1]);
        }
        self.macro_vel = (1.0/self.macro_den)*sum;
    }

    pub fn calc_macro_den(&mut self) {
        // Comes from Equation 3 of source [1] in README.md
        let mut sum: f32 = 0.;
        for i in 0..self.micro_den.len() {
            sum += self.micro_den[i];
        }
        if sum.is_nan() {
            println!("There was some NaN value at a cell; Crushing all to zero");
            println!("{}", self.macro_den);
            for i in 0..self.micro_den.len() {
                self.micro_den[i] = 0.;
            }
            sum = 0.;
        }
        self.macro_den = sum;
    }
}

