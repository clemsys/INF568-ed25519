use super::Key;
use rug::{integer::Order, ops::Pow, Integer};

fn get_p() -> Integer {
    Integer::from(2).pow(255) - Integer::from(19)
}

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

pub fn get_b() -> EdPoint {
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
    EdPoint::new(x, y)
}

#[derive(Clone)]
pub struct EdPoint {
    x: Integer,
    y: Integer,
    z: Integer,
    t: Integer,
}

impl EdPoint {
    pub fn new(x: Integer, y: Integer) -> Self {
        let p = get_p();
        Self {
            x: x.clone().modulo(&p),
            y: y.clone().modulo(&p),
            z: Integer::from(1),
            t: (x * y).modulo(&p),
        }
    }

    fn double(&mut self) {
        let p = get_p();
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

    fn normalize(&mut self) {
        let p = get_p();
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
}

impl std::ops::Add<&Self> for EdPoint {
    type Output = Self;

    fn add(self, other: &Self) -> Self {
        let p = get_p();
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

impl std::ops::Mul<Integer> for EdPoint {
    type Output = Self;

    // self is P, other is s, output is Q
    fn mul(mut self, mut s: Integer) -> Self {
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
        q
    }
}

impl PartialEq for EdPoint {
    fn eq(&self, other: &Self) -> bool {
        let p = get_p();
        (self.x.clone() * &other.z - other.x.clone() * &self.z).modulo(&p) == Integer::ZERO
            && (self.y.clone() * &other.z - other.y.clone() * &self.z).modulo(&p) == Integer::ZERO
    }
}
