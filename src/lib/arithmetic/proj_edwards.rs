use super::xline_proj_montgomery::scalar_mul;
use super::{super::Key, montgomery::MPoint, proj_montgomery::ProjMPoint};
use rug::{integer::Order, ops::Pow, Integer};

fn get_d() -> Integer {
    Integer::from_str_radix(
        "37095705934669439343138083508754565189542113879843219016388785533085940283555",
        10,
    )
    .unwrap()
}

pub fn get_l() -> Integer {
    Integer::from(2).pow(252)
        + Integer::from_str_radix("27742317777372353535851937790883648493", 10).unwrap()
}

pub fn get_b() -> ProjEdPoint {
    let x = Integer::from_str_radix(
        "15112221349535400772501151409588531511454012693041857206046113283949847762202",
        10,
    )
    .unwrap();
    let y = Integer::from_str_radix(
        "46316835694926478169428394003475163141307993866256225615783033603165251855960",
        10,
    )
    .unwrap();
    ProjEdPoint::new(x, y)
}

#[derive(Clone, Debug)]
pub struct ProjEdPoint {
    x: Integer,
    y: Integer,
    z: Integer,
    t: Integer,
}

impl ProjEdPoint {
    pub fn new(x: Integer, y: Integer) -> Self {
        let p = Self::p();
        Self {
            x: x.clone().modulo(&p),
            y: y.clone().modulo(&p),
            z: Integer::from(1),
            t: (x * y).modulo(&p),
        }
    }

    pub const fn x(&self) -> &Integer {
        &self.x
    }

    pub const fn y(&self) -> &Integer {
        &self.y
    }

    pub const fn z(&self) -> &Integer {
        &self.z
    }

    pub fn p() -> Integer {
        Integer::from(2).pow(255) - Integer::from(19)
    }

    pub fn a() -> Integer {
        Integer::from(486_662)
    }

    pub fn b() -> Integer {
        Integer::from(1)
    }

    // for birational equivalence between Edwards and Montgomery curves
    pub fn root_minus_a_minus_2_mod_p() -> Integer {
        Integer::from_str_radix(
            "6853475219497561581579357271197624642482790079785650197046958215289687604742",
            10,
        )
        .unwrap()
    }

    fn normalize(&mut self) {
        let p = Self::p();
        let z_inv = self.z.clone().secure_pow_mod(&(p.clone() - 2), &p);
        self.x = (self.x.clone() * &z_inv).modulo(&p);
        self.y = (self.y.clone() * &z_inv).modulo(&p);
        self.z = Integer::from(1);
        self.t = (self.x.clone() * &self.y).modulo(&p);
    }

    pub fn encode(&mut self) -> Key {
        self.normalize();
        let mut digits: Key = {
            let mut digits = self.y.to_digits(Order::Lsf);
            digits.resize(32, 0);
            digits.try_into().unwrap()
        };
        digits[31] |= (u8::from(self.x.get_bit(0))) << 7;
        digits
    }

    fn recover_x(y: Integer, sign: bool) -> Result<Integer, ()> {
        let p = Self::p();
        if y >= p {
            Err(())
        } else {
            let d: Integer = get_d();
            let two = Integer::from(2);

            let y2: Integer = y.secure_pow_mod(&two, &p);
            let u: Integer = (y2.clone() - Integer::ONE).modulo(&p);
            let v: Integer = (d * y2 + Integer::ONE).modulo(&p);

            let mut x = {
                let a = (u.clone() * v.clone().secure_pow_mod(&Integer::from(3), &p)).modulo(&p);
                let b = (u.clone() * v.clone().secure_pow_mod(&Integer::from(7), &p))
                    .secure_pow_mod(&((p.clone() - 5) / 8), &p);
                (a * b).modulo(&p)
            };

            let vx2 = (v * x.clone().secure_pow_mod(&two, &p)).modulo(&p);

            if vx2.is_congruent(&u, &p) {
            } else if vx2.is_congruent(&(-u), &p) {
                x = (x * Integer::from(2).secure_pow_mod(&((p.clone() - 1) / 4), &p)).modulo(&p);
            } else {
                return Err(());
            };

            if x == Integer::ZERO && sign {
                Err(())
            } else {
                if !(x.is_congruent_u(u32::from(sign), 2)) {
                    x = -x + &p;
                }
                Ok(x)
            }
        }
    }

    pub fn decode(digits: Key) -> Result<Self, ()> {
        let mut y = Integer::from_digits(&digits, Order::Lsf);
        let sign = y.get_bit(255);
        y.set_bit(255, false);

        match Self::recover_x(y.clone(), sign) {
            Ok(x) => Ok(Self {
                x: x.clone(),
                y: y.clone(),
                z: Integer::from(1),
                t: (x * &y).modulo(&Self::p()),
            }),
            Err(()) => Err(()),
        }
    }
}

impl std::ops::Add<&Self> for ProjEdPoint {
    type Output = Self;

    fn add(self, other: &Self) -> Self {
        let p = Self::p();
        let ta = ((self.y.clone() - &self.x) * (other.y.clone() - &other.x)).modulo(&p);
        let tb = ((self.y + &self.x) * (other.y.clone() + &other.x)).modulo(&p);
        let tc = (Integer::from(2) * &self.t * &other.t * get_d()).modulo(&p);
        let td = (Integer::from(2) * &self.z * &other.z).modulo(&p);
        let te = (tb.clone() - &ta).modulo(&p);
        let tf = (td.clone() - &tc).modulo(&p);
        let tg = (td + tc).modulo(&p);
        let th = (tb + ta).modulo(&p);

        Self {
            x: (te.clone() * &tf).modulo(&p),
            y: (tg.clone() * &th).modulo(&p),
            z: (tf * tg).modulo(&p),
            t: (te * th).modulo(&p),
        }
    }
}

// use Montgomery ladder to compute scalar multiplication in constant time
impl std::ops::Mul<&Integer> for ProjEdPoint {
    type Output = Self;

    // self is P, other is s, output is Q
    fn mul(self, s: &Integer) -> Self {
        let proj_m_point = scalar_mul(
            &Self::p(),
            &Self::a(),
            &Self::b(),
            s,
            &MPoint::try_from(&self).unwrap(),
        );
        let mut proj_ed_point = Self::from(&proj_m_point);
        proj_ed_point.normalize();
        Self::from(&proj_m_point)
    }
}

impl PartialEq for ProjEdPoint {
    fn eq(&self, other: &Self) -> bool {
        let p = Self::p();
        (self.x.clone() * &other.z - other.x.clone() * &self.z).modulo(&p) == Integer::ZERO
            && (self.y.clone() * &other.z - other.y.clone() * &self.z).modulo(&p) == Integer::ZERO
    }
}

// x = x (x + z), y = y (x - z), z = y (x + z), t = x (x - z)
impl From<&ProjMPoint> for ProjEdPoint {
    fn from(point: &ProjMPoint) -> Self {
        let p = Self::p();
        let root = Self::root_minus_a_minus_2_mod_p();
        let x = (((point.x().clone() + point.z()) * point.x()).modulo(&p) * &root).modulo(&p);
        let y = ((point.x().clone() - point.z()) * point.y()).modulo(&p);
        let z = ((point.x().clone() + point.z()) * point.y()).modulo(&p);
        let t = (((point.x().clone() - point.z()) * point.x()).modulo(&p) * &root).modulo(&p);
        Self { x, y, z, t }
    }
}

// x = x (x + 1), y = y (x - 1), z = y (x + 1), t = x (x - 1)
impl From<&MPoint> for ProjEdPoint {
    fn from(point: &MPoint) -> Self {
        let p = Self::p();
        let root = Self::root_minus_a_minus_2_mod_p();
        let x = ((point.x().clone() + Integer::from(1)) * point.x() * &root).modulo(&p);
        let y = ((point.x().clone() - Integer::from(1)) * point.y()).modulo(&p);
        let z = ((point.x().clone() + Integer::from(1)) * point.y()).modulo(&p);
        let t = ((point.x().clone() - Integer::from(1)) * point.x() * &root).modulo(&p);
        Self { x, y, z, t }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn correct_from_mpoint() {
        let b = get_b();
        assert_eq!(ProjEdPoint::from(&MPoint::try_from(&b).unwrap()), b);
    }

    #[test]
    fn correct_from_proj_mpoint() {
        let b = get_b();
        assert_eq!(ProjEdPoint::from(&ProjMPoint::from(&b)), b);
    }

    #[test]
    fn correct_from_cycle() {
        let b = get_b();
        assert_eq!(
            ProjEdPoint::from(&ProjMPoint::from(&MPoint::try_from(&b).unwrap())),
            b
        );
    }

    #[test]
    fn correct_from_reverse_cycle() {
        let b = get_b();
        assert_eq!(
            ProjEdPoint::from(&MPoint::try_from(&ProjMPoint::from(&b)).unwrap()),
            b
        );
    }

    fn equivalent_mul(s: Integer) {
        let b = get_b();
        assert_eq!(b.clone() * &s, b.edwards_mul(s));
    }

    // implement scalar multiplication using Edwards formulas from RFC 8032
    // only for debug purposes, not used in release binaries
    impl ProjEdPoint {
        fn double(&mut self) {
            let p = Self::p();
            let two = Integer::from(2);
            let ta = self.x.clone().secure_pow_mod(&two, &p);
            let tb = self.y.clone().secure_pow_mod(&two, &p);
            let tc = self.z.clone().secure_pow_mod(&two, &p) * &two;
            let th = (ta.clone() + &tb).modulo(&p);
            let te = (-(self.x.clone() + &self.y).secure_pow_mod(&two, &p) + &th).modulo(&p);
            let tg = (ta - tb).modulo(&p);
            let tf = (tc + &tg).modulo(&p);
            let x = (te.clone() * &tf).modulo(&p);
            let y = (tg.clone() * &th).modulo(&p);
            let z = (tf * tg).modulo(&p);
            let t = (te * th).modulo(&p);
            (self.x, self.y, self.z, self.t) = (x, y, z, t);
        }

        // self is P, other is s, output is Q
        fn edwards_mul(mut self, mut s: Integer) -> Self {
            let mut q = Self {
                x: Integer::from(0),
                y: Integer::from(1),
                z: Integer::from(1),
                t: Integer::from(0),
            }; // neutral element
            while s > Integer::ZERO {
                if s.get_bit(0) {
                    q = q + &self;
                }
                self.double();
                s >>= 1;
            }
            q.normalize();
            q
        }
    }

    #[test]
    fn equivalent_mul_1() {
        equivalent_mul(Integer::from(1));
    }

    #[test]
    fn equivalent_mul_big_1() {
        equivalent_mul(
            Integer::from_str_radix(
                "36144925721603087658594284515452164870581325872720374094707712194495455132720",
                10,
            )
            .unwrap(),
        );
    }
}
