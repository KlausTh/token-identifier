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

Example
-------
 > hfmon16

Token Identifier
================

Token identifier means a list of token separated with char '-'. The size of a token id is a multiple of 32 bits.

BNF
---

    <tokenid> ::= <token> | <token> '-' <tokenid>

Examples
--------

64 bit ID
 > rvhlgd0-a2m76jr

128 bit ID
 > r76d07p-1k4b3as-158sk46-r8ue8mt

256 bit ID
 > 36lkic1-u9toc59-f4dosvu-is08cmq-37s9a6f-llvjs1o-vbd9soi-qssbj2c
