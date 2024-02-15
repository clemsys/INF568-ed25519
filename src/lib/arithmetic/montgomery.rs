use super::{proj_edwards::ProjEdPoint, proj_montgomery::ProjMPoint};
use rug::Integer;

#[derive(Clone)]
pub struct MPoint {
    x: Integer,
    y: Integer,
}

impl MPoint {
    pub fn new(x: Integer, y: Integer) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> &Integer {
        &self.x
    }

    pub fn y(&self) -> &Integer {
        &self.y
    }
}

impl From<&ProjMPoint> for MPoint {
    fn from(point: &ProjMPoint) -> Self {
        let p = ProjEdPoint::p();
        let x = (point.x().clone() * point.z().clone().invert(&p).unwrap()).modulo(&p);
        let y = (point.y().clone() * point.z().clone().invert(&p).unwrap()).modulo(&p);
        Self::new(x, y)
    }
}

// x = (z + y) / (z - y), y = ((z + y) * z) / ((z - y) * x)
impl From<&ProjEdPoint> for MPoint {
    fn from(point: &ProjEdPoint) -> Self {
        let p = ProjEdPoint::p();
        let root = ProjEdPoint::root_minus_a_minus_2_mod_p();
        let x = ((point.z().clone() + point.y())
            * (point.z().clone() - point.y()).invert(&p).unwrap())
        .modulo(&p);
        let y = ((((point.z().clone() + point.y())
            * (point.z().clone() - point.y()).invert(&p).unwrap())
        .modulo(&p)
            * (point.x().clone().invert(&p).unwrap() * point.z()).modulo(&p))
        .modulo(&p)
            * root)
            .modulo(&p);
        Self::new(x, y)
    }
}
