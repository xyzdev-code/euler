use colour::red;
use std::{iter::Enumerate, process::exit, str::Chars, vec};
#[derive(Debug, PartialEq, Eq)]
pub enum Tokens {
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
fn next_char_equal(i: &mut Enumerate<Chars>, pos: usize, ch: char) -> bool {
    return i.nth(pos).eq(&Some((pos, ch)));
}
pub fn tokenize(program: String) -> Vec<Tokens> {
    let mut final_tokens: Vec<Tokens> = vec![];
    let line_iter = program.lines().enumerate();
    line_iter.for_each(|line| {
        let char_iter = line.1.chars().enumerate();
        let mut char_iter_copy = char_iter.clone();
        let mut skip = false;
        char_iter.for_each(|ch| {
            if skip == false {
                match ch.1 {
                    '(' => final_tokens.push(Tokens::LPAREN),
                    '{' => final_tokens.push(Tokens::LBRACE),
                    ')' => final_tokens.push(Tokens::RPAREN),
                    '}' => final_tokens.push(Tokens::RBRACE),
                    ',' => final_tokens.push(Tokens::COMMA),
                    '.' => final_tokens.push(Tokens::DOT),
                    '-' => final_tokens.push(Tokens::MINUS),
                    '+' => final_tokens.push(Tokens::PLUS),
                    '*' => final_tokens.push(Tokens::STAR),
                    '/' => {
                        if next_char_equal(&mut char_iter_copy, ch.0 + 1, '/') {
                            final_tokens.push(Tokens::COMMENT);
                            skip = true
                        } else {
                            final_tokens.push(Tokens::SLASH)
                        }
                        final_tokens.push(Tokens::SLASH)
                    }
                    ':' => final_tokens.push(Tokens::COLON),
                    '=' => {
                        if next_char_equal(&mut char_iter_copy, ch.0 + 1, '=') {
                            final_tokens.push(Tokens::EQUALS);
                            skip = true;
                        } else {
                            final_tokens.push(Tokens::EQUAL)
                        }
                    }
                    _ => {
                        red!("Syntax Error: ");
                        println!(
                            "Unexpected token at line {} character {}.
        {}| {}
           {}^-- Unexpected '{}' token.",
                            line.0 + 1,
                            ch.0 + 1,
                            line.0 + 1,
                            line.1,
                            " ".repeat(ch.0),
                            ch.1
                        );
                        exit(1)
                    }
                }
            } else {
                skip = false;
            }
        });
    });
    return final_tokens;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_tokenization() {
        assert_eq!(tokenize("==".to_string()), vec!(Tokens::EQUALS));
        assert_eq!(
            tokenize("()".to_string()),
            vec!(Tokens::LPAREN, Tokens::RPAREN)
        )
    }
}
