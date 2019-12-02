pub const SCALE_X: u32 = 8;
pub const SCALE_Y: u32 = 8;

pub const U_X: usize = 32;
pub const U_Y: usize = 32;
pub const X: u32 = U_X as u32;
pub const Y: u32 = U_Y as u32;
pub const C: f32 = 0.2;
pub const T: f32 = 1.0;

pub const MAX_VEL: f32 =  1024.0;
pub const MIN_VEL: f32 = -1024.0;
pub const MAX_DEN: f32 = 256.0;
pub const MIN_DEN: f32 = 0.0;

// Microscopic velocities
pub(crate)pub(crate) static E: [[f32; 2]; 9] = [[0.,0.],
    [1.,0.], [ 0.,1.], [-1., 0.], [0.,-1.],
    [1.,1.], [-1.,1.], [-1.,-1.], [1.,-1.]];
pub(crate) static W: [f32; 9] = [4./9.,
    1./9.,  1./9.,  1./9.,  1./9.,
    1./36., 1./36., 1./36., 1./36.];