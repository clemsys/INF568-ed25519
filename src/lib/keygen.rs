use super::arithmetic::proj_edwards::get_b;
use super::Key;
use rand::RngCore;
use rug::{integer::Order, Integer};
use sha2::{Digest, Sha512};

fn prune(key: &mut Key) {
    key[0] &= 0b1111_1000;
    key[31] &= 0b0111_1111;
    key[31] |= 0b0100_0000;
}

// returns (public, s, hash[32..64])
pub(crate) fn gen_public_scalar_prefix(private: Key) -> (Key, Integer, Key) {
    let hash = Sha512::digest(private);

    let upper_bytes = hash[32..64].try_into().unwrap();

    let mut lower_bytes = hash[0..32].try_into().unwrap();
    prune(&mut lower_bytes);
    let scalar = Integer::from_digits(&lower_bytes, Order::Lsf);

    let public = {
        let b = get_b();
        (b * &scalar).encode()
    };

    (public, scalar, upper_bytes)
}

fn gen_public(private: Key) -> Key {
    let (public, _, _) = gen_public_scalar_prefix(private);
    public
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

    fn key_from_str(s: &str) -> Key {
        s.chars()
            .collect::<Vec<char>>()
            .chunks(2)
            .map(|chunk| chunk.iter().collect::<String>())
            .map(|byte| u8::from_str_radix(&byte, 16).unwrap())
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap()
    }

    fn correct_gen_public(private: &str, expected_public: &str) {
        let private = key_from_str(private);
        let expected_public = key_from_str(expected_public);
        assert_eq!(gen_public(private), expected_public);
    }

    #[test]
    fn correct_gen_public_1() {
        let private = "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60";
        let expected_public = "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a";
        correct_gen_public(private, expected_public);
    }

    #[test]
    fn correct_gen_public_2() {
        let private = "4ccd089b28ff96da9db6c346ec114e0f5b8a319f35aba624da8cf6ed4fb8a6fb";
        let expected_public = "3d4017c3e843895a92b70aa74d1b7ebc9c982ccf2ec4968cc0cd55f12af4660c";
        correct_gen_public(private, expected_public);
    }

    #[test]
    fn correct_gen_public_3() {
        let private = "c5aa8df43f9f837bedb7442f31dcb7b166d38535076f094b85ce3a2e0b4458f7";
        let expected_public = "fc51cd8e6218a1a38da47ed00230f0580816ed13ba3303ac5deb911548908025";
        correct_gen_public(private, expected_public);
    }

    #[test]
    fn correct_gen_public_1024() {
        let private = "f5e5767cf153319517630f226876b86c8160cc583bc013744c6bf255f5cc0ee5";
        let expected_public = "278117fc144c72340f67d0f2316e8386ceffbf2b2428c9c51fef7c597f1d426e";
        correct_gen_public(private, expected_public);
    }

    #[test]
    fn correct_gen_public_sha() {
        let private = "833fe62409237b9d62ec77587520911e9a759cec1d19755b7da901b96dca3d42";
        let expected_public = "ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf";
        correct_gen_public(private, expected_public);
    }
}
