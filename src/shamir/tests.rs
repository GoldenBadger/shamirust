use super::*;

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
