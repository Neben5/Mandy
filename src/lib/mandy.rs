use crate::lib::imag::Imaginary;

#[derive(Clone, Copy)]
pub struct Mandy {
    pub c: Imaginary,
    pub z: Imaginary,
    pub niter: usize,
}

impl Mandy {
    pub fn new(i: Imaginary) -> Mandy {
        return Mandy {
            c: i,
            z: Imaginary::zero(),
            niter: 0,
        };
    }

    pub fn iter(mut self) -> Imaginary {
        self.z = self.z.sq() + self.c;
        return self.z;
    }

    pub fn isdiv(self) -> bool {
        return (self.z.wsq()) >= 4.;
    }
}

pub struct ManCanvas {
    pub elems: Vec<Vec<Mandy>>,
}
