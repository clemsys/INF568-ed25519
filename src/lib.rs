pub mod lib {
    pub type Key = [u8; 32];
    pub type Signature = [u8; 64];

    mod arithmetic;
    pub mod ed25519_keygen;
    pub mod ed25519_sign;
    pub mod ed25519_verify;
}
