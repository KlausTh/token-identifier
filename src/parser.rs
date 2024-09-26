
use crate::{Token, TokenId, RADIX};
use regex::Regex;

pub fn parse_token(token : &str) -> Result<Token, &str> {
    let filter = Regex::new("[0-9a-v]{7}").unwrap();

    if filter.is_match(token) {
        let block : u64 = token.chars()
            .map(|c| u64::from(c.to_digit(RADIX).unwrap()))
            .reduce(|d1,d2| (d1 << 5) + d2).expect("value with checksum");
        let result = Token::create((block & u64::from(u32::MAX)).try_into().unwrap());

        if result.block() == block {
            Ok(result)
        } else {
            Err("Token checksum is not valid")
        }
    } else {
        Err("Token length have to be 7 and chars 0-9 and a-v allowed")
    }
}

pub fn parse_token_id(tokenid : &str) -> Result<TokenId, &str> {
    let res : Vec<Result<Token,&str>> = tokenid.split('-').map(|t| Token::decode(t)).collect();

    if res.iter().all(|r| r.is_ok()) {
        let token : Vec<Token> = res.iter().map(|r| r.unwrap()).collect();

        Ok(TokenId::create(token.into_boxed_slice()))
    } else {
        Err(res.iter().find(|r| r.is_err()).unwrap().unwrap_err())
    }
}
