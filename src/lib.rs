pub mod lib {
    pub type Key = [u8; 32];
    pub type Signature = [u8; 64];

    mod arithmetic {
        mod montgomery;
        pub mod proj_edwards;
        mod proj_montgomery;
        mod xline_proj_montgomery;
    }
    pub mod keygen;
    pub mod sign;
    pub mod verify;
}
