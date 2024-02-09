use rug::{ops::Pow, Integer};

#[derive(Clone)]
struct EdPoint {
    x: Integer,
    y: Integer,
    z: Integer,
    t: Integer,
}

impl EdPoint {
    fn p() -> Integer {
        Integer::from(2).pow(255) - Integer::from(19)
    }

    fn d() -> Integer {
        Integer::from_str_radix(
            "37095705934669439343138083508754565189542113879843219016388785533085940283555",
            10,
        )
        .unwrap()
    }
}

impl std::ops::Add<&Self> for EdPoint {
    type Output = Self;

    fn add(self, other: &Self) -> Self {
        let p = Self::p();
        let ta = ((self.y.clone() - &self.x) * (other.y.clone() - &other.x)).modulo(&p);
        let tb = ((self.y.clone() + &self.x) * (other.y.clone() + &other.x)).modulo(&p);
        let tc = (Integer::from(2) * &self.t * &other.t * Self::d()).modulo(&p);
        let td = (Integer::from(2) * &self.z * &other.z).modulo(&p);
        let te = tb.clone() - &ta;
        let tf = td.clone() - &tc;
        let tg = td + tc;
        let th = tb + ta;

        Self {
            x: te.clone() * &tf,
            y: tg.clone() * &th,
            z: tf * tg,
            t: te * th,
        }
    }
}

impl std::ops::Mul<&Integer> for EdPoint {
    type Output = Self;

    // self is P, other is s, output is Q
    fn mul(self, other: &Integer) -> Self {
        let mut p = self.clone();
        let mut s = other.clone();
        let mut q = EdPoint {
            x: Integer::from(0),
            y: Integer::from(1),
            z: Integer::from(1),
            t: Integer::from(0),
        };
        while s > Integer::ZERO {
            if s.get_bit(0) {
                q = q + &p;
            }
            p = p.clone() + &p;
            s >>= 1;
        }
        q
    }
}
