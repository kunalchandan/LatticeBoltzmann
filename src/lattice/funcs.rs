use crate::lattice::Lattice;
use crate::lattice::consts::*;
use std::f32::consts::PI;

impl Lattice {
    pub(crate) fn add_stuff(&mut self, cursor: [f64; 2]) {
        let c0 = cursor[0] as u32;
        let c1 = cursor[1] as u32;
        println!("{}, {}", (c0/SCALE_X) as usize, (c1/SCALE_Y) as usize);
        for den in 0..self.nodes[(c0/SCALE_X) as usize][(c1/SCALE_Y) as usize].micro_den.len() {
            self.nodes[(c0/SCALE_X) as usize][(c1/SCALE_Y) as usize].micro_den[den] += 1.;
        }
        // Some hope to maybe cause forces to move in random-ish directions
        for vel in 0..self.nodes[(c0/SCALE_X) as usize][(c1/SCALE_Y) as usize].micro_vel.len() {
            self.nodes[(c0/SCALE_X) as usize][(c1/SCALE_Y) as usize].micro_vel[vel] = 10. * unsafe { ((vel as f32) * PI/9.0).cos() };
        }
        println!("{}", self.nodes[(c0/SCALE_X) as usize][(c1/SCALE_Y) as usize].macro_vel);
        println!("{}", self.nodes[(c0/SCALE_X) as usize][(c1/SCALE_Y) as usize].macro_den);
    }

    pub(crate) fn clamp_values(&mut self) {

        for x in 0..self.nodes.len() {
            for y in 0..self.nodes[x].len() {
                // Clamp Velocities
                for i in 0..self.nodes[x][y].micro_vel.len() {
                    self.nodes[x][y].micro_vel[i] = self.nodes[x][y].micro_vel[i].max(MIN_VEL).min( MAX_VEL);
                }
                // Clamp Densities
                for i in 0..self.nodes[x][y].micro_den.len() {
                    self.nodes[x][y].micro_den[i] = self.nodes[x][y].micro_den[i].max(MIN_DEN).min(MAX_DEN);
                }
            }
        }
    }

    pub(crate) fn calc_macros(&mut self) {
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

    pub(crate) fn streaming(&mut self) {
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
        // First swap adjacent velocities in adjacent nodes
        // Second swap opposite velocities within a node
        for x in 0..self.nodes.len() {
            for y in 0..self.nodes[x].len() {
                // Swap for each of the 9 directions.
                if x > 0 {
                    let _s_                         = self.nodes[ x ][y].micro_vel[3];
                    self.nodes[ x ][y].micro_vel[3] = self.nodes[x-1][y].micro_vel[1];
                    self.nodes[x-1][y].micro_vel[1] = _s_;
                    if y > 0 {
                        let _s_                           = self.nodes[ x ][ y ].micro_vel[6];
                        self.nodes[ x ][ y ].micro_vel[6] = self.nodes[x-1][y-1].micro_vel[8];
                        self.nodes[x-1][y-1].micro_vel[8] = _s_;
                    }
                    if y < U_Y - 1 {
                        let _s_                           = self.nodes[ x ][ y ].micro_vel[7];
                        self.nodes[ x ][ y ].micro_vel[7] = self.nodes[x-1][y+1].micro_vel[5];
                        self.nodes[x-1][y+1].micro_vel[5] = _s_;
                    }
                }
                if x < U_X - 1 {
                    let _s_                         = self.nodes[ x ][y].micro_vel[1];
                    self.nodes[ x ][y].micro_vel[1] = self.nodes[x+1][y].micro_vel[3];
                    self.nodes[x+1][y].micro_vel[3] = _s_;
                    if y > 0 {
                        let _s_                           = self.nodes[ x ][ y ].micro_vel[5];
                        self.nodes[ x ][ y ].micro_vel[5] = self.nodes[x+1][y-1].micro_vel[7];
                        self.nodes[x+1][y-1].micro_vel[7] = _s_;
                    }
                    if y < U_Y - 1 {
                        let _s_                           = self.nodes[ x ][ y ].micro_vel[8];
                        self.nodes[ x ][ y ].micro_vel[8] = self.nodes[x+1][y+1].micro_vel[6];
                        self.nodes[x+1][y+1].micro_vel[6] = _s_;
                    }
                }
                if y > 0 {
                    let _s_                         = self.nodes[x][ y ].micro_vel[2];
                    self.nodes[x][ y ].micro_vel[2] = self.nodes[x][y-1].micro_vel[4];
                    self.nodes[x][y-1].micro_vel[4] = _s_;
                }
                if y < U_Y - 1 {
                    let _s_                         = self.nodes[x][ y ].micro_vel[4];
                    self.nodes[x][ y ].micro_vel[4] = self.nodes[x][y+1].micro_vel[2];
                    self.nodes[x][y+1].micro_vel[2] = _s_;
                }
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

    pub(crate) fn calc_eq_dist(&mut self) {
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

    pub(crate) fn collision(&mut self) {
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
}