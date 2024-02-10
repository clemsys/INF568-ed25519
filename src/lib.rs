pub mod lib {
    pub type Key = [u8; 32];
    pub type Signature = [u8; 64];

    mod arithmetic;
    pub mod keygen;
    pub mod sign;
    pub mod verify;
}
