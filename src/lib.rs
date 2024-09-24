
#[cfg(test)]
mod tests;

use std::fmt;
use std::fmt::{Display, Formatter};
use std::iter::repeat_with;
use std::rc::Rc;
use rand::thread_rng;
use rand::Rng;
use regex::Regex;

const RADIX : u32 = 32;

#[derive(Copy)]
#[derive(Debug)]
pub struct Token {
    value : u32
}

pub struct TokenId {
    tokens : Rc<Box<[Token]>>
}

impl Token {
    pub fn new() -> Token {
        Self::create(thread_rng().gen())
    }

    pub fn create(value : u32) -> Self {
        Token {
            value : value
        }
    }

    pub fn decode(token : &str) -> Result<Self, &str> {
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

    pub fn get_value(&self) -> u32 {
        self.value
    }

    pub fn encode(&self) -> String {
        let mask : u64 = 0b11111;
        let block : u64 = self.block();

        let iter = [30,25,20,15,10,5,0].into_iter()
            .map(|bits| char::from_digit((block >> bits & mask).try_into().unwrap(),RADIX).unwrap());

        String::from_iter(iter)
    }

    fn block(&self) -> u64 {
        (u64::from(self.checksum()) << 32)
            + u64::from(self.value)
    }

    fn checksum(&self) -> u32 {
        let mask : u32 = 0b111;

        (0..11).into_iter()
                .map(|i| self.value >> i*3 & mask)
                .reduce(|c1,c2| c1 ^ c2)
                .expect("build checksum")
    }
}

impl TokenId {
    pub fn new_64() -> TokenId {
        Self::new(2)
    }

    pub fn new_128() -> TokenId {
        Self::new(4)
    }

    pub fn new_256() -> TokenId {
        Self::new(8)
    }

    pub fn new(token_len : usize) -> Self {
        let token : Vec<Token> = repeat_with(|| Token::new()).take(token_len).collect();

        Self::create(token.into_boxed_slice())
    }

    pub fn create(tokens : Box<[Token]>) -> Self {
        TokenId {
            tokens : Rc::new(tokens)
        }
    }

    pub fn decode(tokenid : &str) -> Result<TokenId,&str> {
        let res : Vec<Result<Token,&str>> = tokenid.split('-').map(|t| Token::decode(t)).collect();

        if res.iter().all(|r| r.is_ok()) {
            let token : Vec<Token> = res.iter().map(|r| r.unwrap()).collect();

            Ok(TokenId::create(token.into_boxed_slice()))
        } else {
            Err(res.iter().find(|r| r.is_err()).unwrap().unwrap_err())
        }
    }

    pub fn get_tokens(&self) -> Vec<Token> {
        self.tokens.into_iter()
            .map(|b| *b)
            .collect()
    }

    pub fn get_values(&self) -> Vec<u32> {
        self.tokens.into_iter()
            .map(|t| t.get_value())
            .collect()
    }

    fn encode(&self) -> String {
        self.tokens.into_iter()
            .map(|t| t.encode())
            .reduce(|s1,s2| s1 + "-" + &s2)
            .unwrap()
    }
}

impl Clone for Token {
    fn clone(&self) -> Token {
        *self
    }
}

impl Clone for TokenId {
    fn clone(&self) -> Self {
        Self {
            tokens : self.tokens.clone()
        }
    }
}

impl Default for Token {
    fn default() -> Self {
        Self::create(0)
    }
}

impl Into<u32> for Token {
    fn into(self) -> u32 {
        self.value
    }
}

impl Into<String> for Token {
    fn into(self) -> String {
        self.encode()
    }
}

impl Into<String> for TokenId {
    fn into(self) -> String {
        self.encode()
    }
}

impl From<u32> for Token {
    fn from(value : u32) -> Self {
        Token::create(value)
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.encode())
    }
}

impl Display for TokenId {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.encode())
    }
}