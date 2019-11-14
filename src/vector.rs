use std::ops::*;
use std::cmp::*;
use std::borrow::Borrow;


pub struct Vect2 {
    pub x: f32,
    pub y: f32
}


impl Add for Vect2 {
    type Output = Vect2;
    fn add(self, rhs: Vect2) -> Vect2 {
        Vect2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl AddAssign for Vect2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}


impl Sub for Vect2 {
    type Output = Vect2;
    fn sub(self, rhs: Vect2) -> Vect2 {
        Vect2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl SubAssign for Vect2 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        };
    }
}


impl Div for Vect2 {
    type Output = Vect2;
    fn div(self, rhs: Self) -> Vect2 {
        Vect2 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}
impl DivAssign for Vect2 {
    fn div_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        };
    }
}


impl Mul for Vect2 {
    type Output = Vect2;
    fn mul(self, rhs: Self) -> Vect2 {
        Vect2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}
impl MulAssign for Vect2 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        };
    }
}


impl PartialEq for Vect2 {
    fn eq(&self, rhs: &Self) -> bool {
        return (self.x == rhs.x) && (self.y == rhs.y);
    }
}
impl Eq for Vect2 {}
impl PartialOrd for Vect2 {
    /// Compare the absolute magnitude of the vectors
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        (self.x.powi(2) + self.y.powi(2)).partial_cmp((rhs.x.powi(2) + rhs.y.powi(2)).borrow())
    }
}

impl std::fmt::Display for Vect2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
