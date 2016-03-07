use super::*;
use rand::{Rng, thread_rng};

#[test]
fn test_split_data() {
    let secret: Vec<u8> = Vec::from("Hello World!");
    let result = generate_shares(&secret, 5, 3);
    let shares: Vec<ShamirShare>;
    match result {
        Ok(s) => shares = s,
        Err(err) => panic!("generate_shares returned error: {:?}", err),
    }
    assert_eq!(shares.len(), 5);
}

#[test]
fn test_rebuild_data_good() {
    let secret: Vec<u8> = Vec::from("Hello World!");
    let shares_result = generate_shares(&secret, 5, 3);
    let shares: Vec<ShamirShare>;
    match shares_result {
        Ok(s) => shares = s,
        Err(err) => panic!("generate_shares returned error: {:?}", err),
    }
    assert_eq!(shares.len(), 5);

    let rebuilt_secret: Vec<u8>;
    let rebuilt_secret_result = rebuild_secret(&shares[0..3]);
    match rebuilt_secret_result {
        Ok(s) => rebuilt_secret = s,
        Err(err) => panic!("rebuild_secret returned an error: {:?}", err),
    }
    assert_eq!(secret, rebuilt_secret);
}

#[test]
#[should_panic]
fn test_rebuild_data_bad() {
    let secret: Vec<u8> = Vec::from("Hello World!");
    let shares_result = generate_shares(&secret, 5, 3);
    let shares: Vec<ShamirShare>;
    match shares_result {
        Ok(s) => shares = s,
        Err(err) => panic!("generate_shares returned error: {:?}", err),
    }
    assert_eq!(shares.len(), 5);

    let rebuilt_secret: Vec<u8>;
    let rebuilt_secret_result = rebuild_secret(&shares[0..2]);
    match rebuilt_secret_result {
        Ok(s) => rebuilt_secret = s,
        Err(err) => panic!("rebuild_secret returned an error: {:?}", err),
    }
    assert_eq!(secret, rebuilt_secret);
}

#[test]
fn test_rebuild_data_non_contiguous() {
    let secret: Vec<u8> = Vec::from("Hello World!");
    let shares_result = generate_shares(&secret, 5, 3);
    let mut shares: Vec<ShamirShare>;
    match shares_result {
        Ok(s) => shares = s,
        Err(err) => panic!("generate_shares returned error: {:?}", err),
    }
    assert_eq!(shares.len(), 5);

    shares.remove(1);
    shares.remove(2);
    let rebuilt_secret: Vec<u8>;
    let rebuilt_secret_result = rebuild_secret(shares.as_slice());
    match rebuilt_secret_result {
        Ok(s) => rebuilt_secret = s,
        Err(err) => panic!("rebuild_secret returned an error: {:?}", err),
    }
    assert_eq!(secret, rebuilt_secret);
}

#[test]
fn test_rebuild_data_unsorted() {
    let secret: Vec<u8> = Vec::from("Hello World!");
    let shares_result = generate_shares(&secret, 5, 3);
    let mut shares: Vec<ShamirShare>;
    match shares_result {
        Ok(s) => shares = s,
        Err(err) => panic!("generate_shares returned error: {:?}", err),
    }
    assert_eq!(shares.len(), 5);

    thread_rng().shuffle(shares.as_mut_slice());
    let rebuilt_secret: Vec<u8>;
    let rebuilt_secret_result = rebuild_secret(shares.as_slice());
    match rebuilt_secret_result {
        Ok(s) => rebuilt_secret = s,
        Err(err) => panic!("rebuild_secret returned an error: {:?}", err),
    }
    assert_eq!(secret, rebuilt_secret);
}
