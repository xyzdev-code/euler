use std::{iter::Enumerate, str::Chars, vec};
#[derive(Debug)]
pub enum Tokens{
    // Single Char tokens
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SLASH,
    STAR,
    COLON,
    EQUAL,
    // two char tokens
    EQUALS,
    NEQUAL,
    GREATER,
    GEQUAL,
    LESS,
    LEQUAL,
    ARROW,
    CBRACE,
    COMMENT,
    // literals
    IDENTIFIER,
    STRING,
    NUMBER,
    // keywords
    MATCH,
    IF,
    ELSEIF,
    ELSE,
}
fn next_char_equal(i: &mut Enumerate<Chars>, pos: usize, ch:char)->bool{
    return i.nth(pos).eq(&Some((pos, ch)))
}
pub fn tokenize(program: String)->Vec<Tokens>{
    let mut final_tokens: Vec<Tokens> = vec![];
    let line_iter = program.lines().enumerate();
    line_iter.for_each(|line|{
        let char_iter = line.1.chars().enumerate();
        let mut ptr_char_iter = char_iter.clone();
        char_iter.for_each(|ch|{
            match ch.1 {
                '(' => final_tokens.push(Tokens::LPAREN),
                ')' => final_tokens.push(Tokens::RPAREN),
                '{' => final_tokens.push(Tokens::LBRACE),
                '}' => final_tokens.push(Tokens::RBRACE),
                ',' => final_tokens.push(Tokens::COMMA),
                '.' => final_tokens.push(Tokens::DOT),
                '-' => final_tokens.push(Tokens::MINUS),
                '+' => final_tokens.push(Tokens::PLUS),
                '*' => final_tokens.push(Tokens::STAR),
                '/' => final_tokens.push(Tokens::SLASH),
                ':' => final_tokens.push(Tokens::COLON),
                '=' => {
                    if next_char_equal(&mut ptr_char_iter, &ch.0+1, '='){
                        final_tokens.push(Tokens::EQUALS)
                    }else{
                        final_tokens.push(Tokens::EQUAL)
                    }
                },
                _ => panic!(
                    "Syntax error: Unexpected token at line {} character {}.
{}|{}
   {}^-- Unexpected '{}' token.
                ", line.0 + 1, ch.0 + 1, line.0 + 1, line.1, " ".repeat(ch.0), ch.1)
            }
        })
    });
    return final_tokens
}
