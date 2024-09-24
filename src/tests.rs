
use crate::*;

#[test]
fn min_token() {
    let token = Token::create(u32::MIN);

    assert_eq!(token.encode(), "0000000");
}

#[test]
fn max_token() {
    let token = Token::create(u32::MAX);

    assert_eq!(token.encode(), "fvvvvvv");
}

#[test]
fn zero_tid_64() {
    let n = TokenId::create(Box::new([Token::create(0);2]));
    let s : String = n.into();

    assert_eq!(s, "0000000-0000000");
}

#[test]
fn zero_tid_128() {
    let n = TokenId::create(Box::new([Token::create(0);4]));
    let s : String = n.into();

    assert_eq!(s, "0000000-0000000-0000000-0000000");
}

#[test]
fn zero_tid_256() {
    let n = TokenId::create(Box::new([Token::create(0);8]));
    let s : String = n.into();

    assert_eq!(s, "0000000-0000000-0000000-0000000-0000000-0000000-0000000-0000000");
}

#[test]
fn token_encode_decode() {
    let t0 = Token::new();
    let s0 = t0.encode();
    let t1 = Token::decode(&s0).expect("valid token &str");
    let s1 = t1.encode();

    assert_eq!(s0, s1);
}

#[test]
fn tokenid_decode() {
    let id0 = TokenId::decode("0000000-0000000-0000000").unwrap();
    assert_eq!(id0.get_values(), [0,0,0]);

    let id1 = TokenId::decode("fvvvvvv-fvvvvvv").unwrap();
    assert_eq!(id1.get_values(), [u32::MAX;2]);
}