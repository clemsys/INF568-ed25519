use super::{montgomery::MPoint, proj_edwards::ProjEdPoint};
use rug::Integer;

#[derive(Clone)]
pub struct ProjMPoint {
    x: Integer,
    y: Integer,
    z: Integer,
}

impl ProjMPoint {
    pub fn new(x: Integer, y: Integer, z: Integer) -> Self {
        Self { x, y, z }
    }

    pub fn x(&self) -> &Integer {
        &self.x
    }

    pub fn y(&self) -> &Integer {
        &self.y
    }

    pub fn z(&self) -> &Integer {
        &self.z
    }
}

impl From<&MPoint> for ProjMPoint {
    fn from(p: &MPoint) -> Self {
        Self::new(p.x().clone(), p.y().clone(), Integer::from(1))
    }
}

// x = (z + y) x, y = (z + y) z, z = (z - y) x
impl From<&ProjEdPoint> for ProjMPoint {
    fn from(point: &ProjEdPoint) -> Self {
        let p = ProjEdPoint::p();
        let root = ProjEdPoint::root_minus_a_minus_2_mod_p();
        let x = ((point.z().clone() + point.y()) * point.x()).modulo(&p);
        let y = (((point.z().clone() + point.y()) * point.z()).modulo(&p) * root).modulo(&p);
        let z = ((point.z().clone() - point.y()) * point.x()).modulo(&p);
        Self::new(x, y, z)
    }
}
