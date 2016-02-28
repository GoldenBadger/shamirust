pub mod shamir {
    use std::result;
    use num::{BigUint, Zero, One};

    const PRIME_SIZE: u16 = 320;

    pub struct ShamirShare {
        pub data: Vec<u8>,
        prime: BigUint, // The prime used to generate this share
        input: u64, // The input to the polynomial which gives "data" as the output
    }

    pub enum ShamirError {
        InvalidArgument(String),
        PrimeMismatch,
    }

    pub type Result<T> = result::Result<T, ShamirError>;

    pub fn generate_shares(secret: Vec<u8>) -> Result<Vec<ShamirShare>> {}
}
