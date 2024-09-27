#[cfg(test)]
mod tests;

mod parser;

use parser::{parse_token, parse_token_id};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::iter::repeat_with;
use std::rc::Rc;
use rand::thread_rng;
use rand::Rng;

/// radix for convert the digits 0-9,a-v
const RADIX : u32 = 32;

/// simple 32 bit token, that can be displayed as a 7 char string.
#[derive(Copy)]
#[derive(Debug)]
pub struct Token {
    value : u32
}

/// a token identifier, that is a combination of 1 to n 32 bit token
#[derive(Debug)]
pub struct TokenId {
    tokens : Rc<Box<[Token]>>
}

impl Token {
    /// generate a token with random value
    pub fn new() -> Self {
        Self::create(thread_rng().gen())
    }

    /// generate a token with a specific value
    pub fn create(value : u32) -> Self {
        Token {
            value : value
        }
    }

    /// parse a token string
    pub fn decode(token : &str) -> Result<Self, &str> {
        parse_token(token)
    }

    /// get token value
    pub fn get_value(&self) -> u32 {
        self.value
    }

    /// calculate die string representation with error correction
    pub fn encode(&self) -> String {
        let mask : u64 = 0b11111;
        let block : u64 = self.block();

        let iter = [30,25,20,15,10,5,0].into_iter()
            .map(|bits| char::from_digit((block >> bits & mask).try_into().unwrap(),RADIX).unwrap());

        String::from_iter(iter)
    }

    /// generate 35 bit block of value and check bits
    fn block(&self) -> u64 {
        (u64::from(self.checksum()) << 32)
            + u64::from(self.value)
    }

    /// checksum for token is a 3 bit value
    fn checksum(&self) -> u32 {
        let mask : u32 = 0b111;

        (0..11).into_iter()
                .map(|i| self.value >> i*3 & mask)
                .reduce(|c1,c2| c1 ^ c2)
                .expect("build checksum")
    }
}

impl TokenId {
    /// generate a 64 bit token identifier with random value
    pub fn new_64() -> Self {
        Self::new(2)
    }

    /// generate a 128 bit token identifier with random value
    pub fn new_128() -> Self {
        Self::new(4)
    }

    /// generate a 256 bit token identifier with random value
    pub fn new_256() -> Self {
        Self::new(8)
    }

    /// generate a token identifier with random value of variable length
    pub fn new(token_len : usize) -> Self {
        let token : Vec<Token> = repeat_with(|| Token::new()).take(token_len).collect();

        Self::create(token.into_boxed_slice())
    }

    /// generate a token identifier with spezific value
    pub fn create(tokens : Box<[Token]>) -> Self {
        TokenId {
            tokens : Rc::new(tokens)
        }
    }

    /// decode a string to a token identifier
    pub fn decode(tokenid : &str) -> Result<Self,&str> {
        parse_token_id(tokenid)
    }

    /// get list of simple tokens
    pub fn get_tokens(&self) -> Vec<Token> {
        self.tokens.into_iter()
            .map(|b| *b)
            .collect()
    }

    /// get numeric values of the token identifier
    pub fn get_values(&self) -> Vec<u32> {
        self.tokens.into_iter()
            .map(|t| t.get_value())
            .collect()
    }

    /// calculate die string representation of token identifier
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
        Self::create(value)
    }
}

/*
impl From<String> for Token {
    fn from(value : String) -> Self {
        Self::decode(&value)
    }
}

impl From<&str> for Token {
    fn from(value : &str) -> Self {
        Self::decode(value)
    }
}

impl From<String> for TokenId {
    fn from(value : String) -> Self {
        Self::decode(&value)
    }
}

impl From<&str> for TokenId {
    fn from(value : &str) -> Self {
        Self::decode(value)
    }
}
*/

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
