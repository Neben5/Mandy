#![allow(dead_code)]
use std::f64;
use std::ops;

#[derive(Clone, Copy)]
pub struct Imaginary {
    pub a: f64,
    pub b: f64,
}

impl ops::Add<Imaginary> for Imaginary {
    type Output = Imaginary;

    fn add(self, other: Imaginary) -> Self::Output {
        Imaginary {
            a: self.a + other.a,
            b: self.b + other.b,
        }
    }
}

impl ops::Mul<f64> for Imaginary {
    type Output = Imaginary;

    fn mul(self, rhs: f64) -> Self::Output {
        Imaginary {
            a: self.a * rhs,
            b: self.b * rhs,
        }
    }
}

impl ops::Sub<Imaginary> for Imaginary {
    type Output = Imaginary;

    fn sub(self, rhs: Imaginary) -> Self::Output {
        Imaginary {
            a: self.a - rhs.a,
            b: self.b - rhs.b,
        }
    }
}

impl Imaginary {
    pub fn zero() -> Imaginary {
        return Imaginary { a: 0.0, b: 0.0 };
    }

    // return a^2 + b^2
    pub fn wsq(self) -> f64 {
        return (self.a * self.a) + (self.b * self.b);
    }

    pub fn sq(self) -> Imaginary {
        Imaginary {
            a: (self.a * self.a) - (self.b * self.b),
            b: 2. * self.a * self.b,
        }
    }

    pub fn mag(self) -> f64 {
        return ((self.a * self.a) + (self.b * self.b)).sqrt();
    }
}
