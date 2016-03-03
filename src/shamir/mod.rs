#[cfg(test)]
mod tests;

use gmp::{mpz, rand};
use std::result;

pub struct ShamirShare {
    pub data: mpz::Mpz,
    pub prime: mpz::Mpz, // The prime used to generate this share
    pub input: u64, // The input to the polynomial which gives "data" as the output
}

#[derive(Debug)]
pub enum ShamirError {
    DuplicateInput,
    InvalidArgument(String),
    PrimeMismatch,
}

pub type Result<T> = result::Result<T, ShamirError>;

pub fn generate_shares(secret: &Vec<u8>,
                       num_pieces: u64,
                       required_pieces: u64)
                       -> Result<Vec<ShamirShare>> {
    if required_pieces > num_pieces {
        return Err(ShamirError::InvalidArgument("required_pieces must be less than or equal to \
                                                 num_pieces"
                                                    .to_string()));
    }
    if required_pieces <= 0 || num_pieces <= 0 {
        return Err(ShamirError::InvalidArgument("required_pieces or num_pieces cannot be less \
                                                 than or equal to 0"
                                                    .to_string()));
    }
    let prime_size: u64 = (secret.len() as f64 * 1.1 * 8.0) as u64;
    let mut randstate = rand::RandState::new_mt();
    let mut prime = randstate.urandom_2exp(prime_size);
    prime = prime.nextprime();

    let mut coefficients: Vec<mpz::Mpz> = Vec::new();
    let big_secret = mpz::Mpz::from(secret);
    coefficients.push(big_secret);

    for _ in 1..required_pieces {
        let tmp = randstate.urandom(&prime);
        coefficients.push(tmp);
    }

    let mut pieces: Vec<ShamirShare> = Vec::new();
    for x in 1..num_pieces + 1 {
        let mut total = mpz::Mpz::zero();
        for i in 0..required_pieces as usize {
            let degree_total = &coefficients[i] * (mpz::Mpz::from(x).pow(i as u32));
            total = total + degree_total;
        }
        total = total.modulus(&prime);
        pieces.push(ShamirShare {
            data: total,
            prime: prime.clone(),
            input: x,
        });
    }
    Ok(pieces)
}
