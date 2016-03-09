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
    InvalidArgument(String),
    PrimeMismatch,
}

pub type Result<T> = result::Result<T, ShamirError>;

/// Generates a Vec<ShamirShare> from a given secret.
///
/// # Failures
///
/// Returns ShamirError::InvalidArgument when the number of pieces generated is
/// less than the number of pieces required to rebuild, and when either num_pieces
/// or required_pieces is 0.
///
/// # Examples
///
/// ```
/// use shamir::{ShamirShare, generate_shares};
///
/// let data: Vec<u8> = Vec::from("Hello World");
/// let result = generate_shares(data.as_slice(), 5, 3);
/// let shares: Vec<ShamirShare>;
/// match result {
///     Ok(s) => shares = s,
///     Err(err) => panic!("could not generate shares"),
/// }
/// ```
pub fn generate_shares(secret: &[u8],
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

pub fn rebuild_secret(shares: &[ShamirShare]) -> Result<Vec<u8>> {
    let prime = &shares[0].prime;
    for share in shares {
        if *prime != share.prime {
            return Err(ShamirError::PrimeMismatch);
        }
    }
    let mut inputs: Vec<i64> = Vec::new();
    for i in 0..shares.len() {
        inputs.push(shares[i].input as i64);
    }

    let mut sum = mpz::Mpz::zero();
    for i in 0..shares.len() {
        let mut numerator = mpz::Mpz::one();
        let mut denominator = mpz::Mpz::one();
        for j in 0..shares.len() {
            if j == i {
                continue;
            }
            numerator = (numerator * mpz::Mpz::from(-inputs[j])) % prime;
            denominator = (denominator * mpz::Mpz::from(inputs[i] - inputs[j])) % prime;
        }
        let tmp = &shares[i].data * numerator * denominator.invert(prime).unwrap();
        sum = (sum + prime + tmp) % prime;
    }
    Ok(Into::<Option<Vec<u8>>>::into(&sum).unwrap())
}
