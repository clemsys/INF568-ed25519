use rand::RngCore;
use rug::Integer;
use sha2::{Digest, Sha512};

pub type Key = [u8; 32];

fn prune(key: &mut Key) {
    key[0] &= 0b1111_1000;
    key[31] &= 0b0011_1111;
}

fn gen_public(private: Key) -> Key {
    let mut public = Sha512::digest(private)[0..32].try_into().unwrap();

    prune(&mut public);

    let b = (
        Integer::from_str_radix(
            "15112221349535400772501151409588531511454012693041857206046113283949847762202",
            10,
        ),
        Integer::from_str_radix(
            "46316835694926478169428394003475163141307993866256225615783033603165251855960",
            10,
        ),
    );
    todo!()
}

pub fn generate_key_pair() -> (Key, Key) {
    let private = {
        let mut private = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut private);
        private
    };

    (private, gen_public(private))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn correct_gen_public_1() {
        let private: Key = [
            0x9d, 0x61, 0xb1, 0x9d, 0xef, 0xfd, 0x5a, 0x60, 0xba, 0x84, 0x4a, 0xf4, 0x92, 0xec,
            0x2c, 0xc4, 0x44, 0x49, 0xc5, 0x69, 0x7b, 0x32, 0x69, 0x19, 0x70, 0x3b, 0xac, 0x03,
            0x1c, 0xae, 0x7f, 0x60,
        ];
        let expected_public: Key = [
            0xd7, 0x5a, 0x98, 0x01, 0x82, 0xb1, 0x0a, 0xb7, 0xd5, 0x4b, 0xfe, 0xd3, 0xc9, 0x64,
            0x07, 0x3a, 0x0e, 0xe1, 0x72, 0xf3, 0xda, 0xa6, 0x23, 0x25, 0xaf, 0x02, 0x1a, 0x68,
            0xf7, 0x07, 0x51, 0x1a,
        ];
        assert_eq!(gen_public(private), expected_public);
    }
}
