
use token_identifier::{Token, TokenId};

fn main() {
    let token = Token::new();

    println!("simple 32 bit token : {:?}", token);

    let id = TokenId::new_128();

    println!("token id with 128 bits : {:?}", id);
}