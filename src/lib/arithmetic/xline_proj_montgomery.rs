use rug::Integer;

use super::{montgomery::MPoint, proj_edwards::ProjEdPoint, proj_montgomery::ProjMPoint};

#[derive(Clone)]
pub struct XLineProjMPoint {
    x: Integer,
    z: Integer,
}

impl std::ops::Mul<bool> for XLineProjMPoint {
    type Output = Self;
    /// constant time multiplication of a `XLineProjMPoint` by a bool
    fn mul(self, rhs: bool) -> Self {
        let f = u32::from(rhs);
        let zero = Self {
            x: Integer::from(0),
            z: Integer::from(0),
        };
        Self {
            x: self.x * f + zero.x * (1 - f),
            z: self.z * f + zero.z * (1 - f),
        }
    }
}

impl std::ops::Add<Self> for XLineProjMPoint {
    type Output = Self;
    /// coordinate-wise addition of two `XLineProjMPoint`
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            z: self.z + rhs.z,
        }
    }
}

impl From<ProjEdPoint> for XLineProjMPoint {
    /// convert an `EdPoint` to a `XLineProjMPoint`
    fn from(point: ProjEdPoint) -> Self {
        Self {
            x: point.z().clone() + point.y(),
            z: point.z().clone() - point.y(),
        }
    }
}

impl XLineProjMPoint {
    pub const fn x(&self) -> &Integer {
        &self.x
    }

    pub const fn z(&self) -> &Integer {
        &self.z
    }

    /// coordinate-wise modulo of a `XLineProjMPoint`
    fn modulo(self, p: &Integer) -> Self {
        Self {
            x: self.x.modulo(p),
            z: self.z.modulo(p),
        }
    }

    /// replace x by x/z and z by 1
    fn normalize(self, p: &Integer) -> Self {
        Self {
            x: (self.x * self.z.invert(p).unwrap()).modulo(p),
            z: Integer::from(1),
        }
    }
}

// pseudo-add for Montgomery ladder
fn x_add(
    p: &Integer,
    x_p: &XLineProjMPoint,
    x_q: &XLineProjMPoint,
    x_pmq: &XLineProjMPoint,
) -> XLineProjMPoint {
    let two = Integer::from(2);
    let u = ((x_p.x.clone() - &x_p.z).modulo(p) * (x_q.x.clone() + &x_q.z).modulo(p)).modulo(p);
    let v = ((x_p.x.clone() + &x_p.z).modulo(p) * (x_q.x.clone() - &x_q.z).modulo(p)).modulo(p);
    let x = x_pmq.z.clone() * ((u.clone() + &v).secure_pow_mod(&two, p));
    let z = x_pmq.x.clone() * ((u - v).secure_pow_mod(&two, p));
    XLineProjMPoint { x, z }
}

// pseudo-double for Montgomery ladder
fn x_dbl(p: &Integer, a: &Integer, x_p: &XLineProjMPoint) -> XLineProjMPoint {
    let two = Integer::from(2);
    let q = (x_p.x.clone() + &x_p.z).secure_pow_mod(&two, p);
    let r = (x_p.x.clone() - &x_p.z).secure_pow_mod(&two, p);
    let s = (q.clone() - &r).modulo(p);
    let x = (q * &r).modulo(p);
    let ap2o4ts = ((((a.clone() + 2) * (Integer::from(4).invert(p).unwrap()).modulo(p)) * &s)
        as Integer)
        .modulo(p);
    let z: Integer = (((r + ap2o4ts) * s) as Integer).modulo(p);
    XLineProjMPoint { x, z }
}

// Montgomery ladder for constant time scalar multiplication
fn ladder(p: &Integer, a: &Integer, m: &Integer, x_p: &Integer) -> (Integer, Integer) {
    let u = XLineProjMPoint {
        x: x_p.clone(),
        z: Integer::from(1),
    };
    let mut x_0 = XLineProjMPoint {
        x: Integer::from(1),
        z: Integer::from(0),
    };
    let mut x_1 = u.clone();
    for i in (0..m.significant_bits()).rev() {
        let add = x_add(p, &x_0, &x_1, &u);
        let dbl_0 = x_dbl(p, a, &x_0);
        let dbl_1 = x_dbl(p, a, &x_1);
        let bit = m.get_bit(i);
        x_0 = (add.clone() * bit + dbl_0 * !bit).modulo(p);
        x_1 = (dbl_1 * bit + add * !bit).modulo(p);
    }
    (x_0.normalize(p).x, x_1.normalize(p).x)
}

// Okeya–Sakurai y-coordinate recovery, returns point q
fn y_recovery(
    p: &Integer, // p = 2^255 - 19 for curve25519
    a: &Integer, // a = 486662 for curve25519
    b: &Integer, // b = 1 for curve25519
    point: &MPoint,
    x_q: &XLineProjMPoint,
    x_plus: &XLineProjMPoint,
) -> ProjMPoint {
    let two = Integer::from(2);
    let mut v1 = (point.x().clone() * x_q.z()).modulo(p);
    let mut v2 = (x_q.x.clone() + &v1).modulo(p);
    let v3 = ((x_q.x.clone() - &v1).secure_pow_mod(&two, p) * x_plus.x()).modulo(p);
    v1 = (Integer::from(2) * a * x_q.z()).modulo(p);
    v2 = ((v2 + &v1) * (point.x().clone() * x_q.x() + x_q.z()).modulo(p)).modulo(p);
    v1 = (v1 * x_q.z()).modulo(p);
    v2 = ((v2 - &v1) * x_plus.z()).modulo(p);
    let y = (v2 - &v3).modulo(p);
    v1 = Integer::from(2) * b * point.y() * x_q.z() * x_plus.z();
    let x = (v1.clone() * x_q.x()).modulo(p);
    let z = (v1 * x_q.z()).modulo(p);
    ProjMPoint::new(x, y, z)
}

// combines ladder and y_recovery to compute scalar multiplication on full montgomery points
pub fn scalar_mul(
    p: &Integer,
    a: &Integer,
    b: &Integer,
    m: &Integer,
    point: &MPoint,
) -> ProjMPoint {
    let (x_0, x_1) = ladder(p, a, m, point.x());
    let x_0_point = XLineProjMPoint {
        x: x_0,
        z: Integer::from(1),
    };
    let x_1_point = XLineProjMPoint {
        x: x_1,
        z: Integer::from(1),
    };
    y_recovery(p, a, b, point, &x_0_point, &x_1_point)
}

impl From<&ProjMPoint> for XLineProjMPoint {
    fn from(p: &ProjMPoint) -> Self {
        Self {
            x: p.x().clone(),
            z: p.z().clone(),
        }
    }
}

impl From<&MPoint> for XLineProjMPoint {
    fn from(p: &MPoint) -> Self {
        Self {
            x: p.x().clone(),
            z: Integer::from(1),
        }
    }
}

impl From<&ProjEdPoint> for XLineProjMPoint {
    fn from(p: &ProjEdPoint) -> Self {
        Self::from(&ProjMPoint::from(p))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_x_dbl() {
        let p = Integer::from(101);
        let a = Integer::from(49);
        let x_p = XLineProjMPoint {
            x: Integer::from(2),
            z: Integer::from(1),
        };
        let dbl = x_dbl(&p, &a, &x_p);
        assert_eq!(dbl.x, Integer::from(9));
        assert_eq!(dbl.z, Integer::from(16));
    }

    fn test_ladder(p: u32, a: u32, m: u32, x: u32, expected: u32) {
        let p = Integer::from(p);
        let a = Integer::from(a);
        let x = Integer::from(x);
        let m = Integer::from(m);
        let expected = Integer::from(expected);
        let (result, _) = ladder(&p, &a, &m, &x);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_ladder_101_2() {
        test_ladder(101, 49, 2, 2, 70);
    }

    #[test]
    fn test_ladder_101_3() {
        test_ladder(101, 49, 3, 2, 59);
    }

    #[test]
    fn test_ladder_101_77() {
        test_ladder(101, 49, 77, 2, 8);
    }

    #[test]
    fn test_ladder_1009_2() {
        test_ladder(1009, 682, 2, 7, 284);
    }

    #[test]
    fn test_ladder_1009_3() {
        test_ladder(1009, 682, 3, 7, 759);
    }

    #[test]
    fn test_ladder_1009_5() {
        test_ladder(1009, 682, 5, 7, 1000);
    }

    #[test]
    fn test_ladder_1009_34() {
        test_ladder(1009, 682, 34, 7, 286);
    }

    #[test]
    fn test_ladder_1009_104() {
        test_ladder(1009, 682, 104, 7, 810);
    }

    #[test]
    fn test_ladder_1009_947() {
        test_ladder(1009, 682, 947, 7, 755);
    }
}
