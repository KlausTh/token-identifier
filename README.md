Introduction
============

Goals for this implementation of tokens

- compact representation
- easy verbally spelling (no upper case)
- adaptable token identifier
- error recognition
- restriction to ASCII chars
    - can be used in URLs without percent encoding

Token
=====

Basic token value is a 32 bit value, plus 3 bits of error recognition. This token is represented with seven digits, each digit encoded five bits.

Example
-------

    use token_identifier::Token;

    fn main() {
        let token = Token::new();

        println!("simple 32 bit token : {}", token);
    }

 > simple 32 bit token : hfmon16

Encoding
--------

    cccvv|vvvvv|vvvvv|vvvvv|vvvvv|vvvvv|vvvvv

* c : 3 check bits
* v : 32 value bits
* | : digit border

BNF
---

    <token> ::= <digit> <digit> <digit> <digit> <digit> <digit> <digit>
    <digit> ::= 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | a | b | c | d | e | f | g | h | i | j | k | l | m | n | o | p | q | r | s | t | u | v

Token Identifier
================

Token identifier means a list of token separated with char '-'. The size of a token id is a multiple of 32 bits.

Example
-------

    use token_identifier::TokenId;

    fn main() {
        let token = TokenId::new_128();

        println!("token id with 128 bits : {}", token);
    }

 > token id with 128 bits : mht6fmh-aputm5h-5ih87pp-upc8sqc

BNF
---

    <tokenid> ::= <token> '-' <tokenid> | <token>
