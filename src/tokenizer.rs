use colour::red;
use std::{iter::Enumerate, process::exit, str::Chars, vec};
#[derive(Debug, PartialEq, Eq)]
pub enum Tokens {
    // Single Char tokens
    LPAREN((usize, usize), Option<String>),
    RPAREN((usize, usize), Option<String>),
    LBRACE((usize, usize), Option<String>),
    RBRACE((usize, usize), Option<String>),
    COMMA((usize, usize), Option<String>),
    DOT((usize, usize), Option<String>),
    MINUS((usize, usize), Option<String>),
    PLUS((usize, usize), Option<String>),
    SLASH((usize, usize), Option<String>),
    STAR((usize, usize), Option<String>),
    COLON((usize, usize), Option<String>),
    EQUAL((usize, usize), Option<String>),
    BANG((usize, usize), Option<String>),
    //(usize, usize),  two char tokens
    EQUALS((usize, usize), Option<String>),
    NEQUAL((usize, usize), Option<String>),
    GREATER((usize, usize), Option<String>),
    GEQUAL((usize, usize), Option<String>),
    LESS((usize, usize), Option<String>),
    LEQUAL((usize, usize), Option<String>),
    ARROW((usize, usize), Option<String>),
    COMMENT((usize, usize), Option<String>),
    FARROW((usize, usize), Option<String>),
    AND((usize, usize), Option<String>),
    OR((usize, usize), Option<String>),
    PIPE((usize, usize), Option<String>),
    // literals
    IDENTIFIER((usize, usize), Option<String>),
    STRING((usize, usize), Option<String>),
    NUMBER((usize, usize), Option<String>),
    // keywords
    MATCH((usize, usize), Option<String>),
    IF((usize, usize), Option<String>),
    ELSEIF((usize, usize), Option<String>),
    ELSE((usize, usize), Option<String>),
    RETURN((usize, usize), Option<String>),
    TRUE((usize, usize), Option<String>),
    FALSE((usize, usize), Option<String>),
    NIL((usize, usize), Option<String>),
}
fn next_char_equal(i: &mut Enumerate<Chars>, pos: usize, ch: char) -> bool {
    return i.nth(pos).eq(&Some((pos, ch)));
}
fn syntax_error_print(line: (usize, &str), ch: (usize, char), msg:Option<String>) {
    let final_msg: String = match msg {
        Some(text) => format!("Unexpected {} character. Consider {}.", ch.1, text),
        None => format!("Unexpected {} character.", ch.1)
    };
    red!("Syntax Error: ");
    println!(
        "Unexpected token at line {} character {}.
        {}| {}
           {}^-- {}",
        line.0 + 1,
        ch.0 + 1,
        line.0 + 1,
        line.1,
        " ".repeat(ch.0),
        final_msg
    );
    exit(1)
}
pub fn tokenize(program: String) -> Vec<Tokens> {
    let mut final_tokens: Vec<Tokens> = vec![];
    let line_iter = program.lines().enumerate();
    let mut skip_line = false;
    line_iter.for_each(|line| {
        let char_iter = line.1.chars().enumerate();
        let mut char_iter_copy = char_iter.clone();
        let mut skip = false;
        char_iter.for_each(|ch| {
            if skip == false && skip_line == false {
                match ch.1 {
                    '(' => final_tokens.push(Tokens::LPAREN((line.0, ch.0), None)),
                    '{' => final_tokens.push(Tokens::LBRACE((line.0, ch.0), None)),
                    ')' => final_tokens.push(Tokens::RPAREN((line.0, ch.0), None)),
                    '}' => final_tokens.push(Tokens::RBRACE((line.0, ch.0), None)),
                    ',' => final_tokens.push(Tokens::COMMA((line.0, ch.0), None)),
                    '.' => final_tokens.push(Tokens::DOT((line.0, ch.0), None)),
                    '-' => {
                        if next_char_equal(&mut char_iter_copy, ch.0 + 1, '>') {
                            final_tokens.push(Tokens::ARROW((line.0, ch.0), None));
                            skip = true;
                        } else {
                            final_tokens.push(Tokens::MINUS((line.0, ch.0), None))
                        }
                    }
                    '+' => final_tokens.push(Tokens::PLUS((line.0, ch.0), None)),
                    '*' => final_tokens.push(Tokens::STAR((line.0, ch.0), None)),
                    '/' => {
                        if next_char_equal(&mut char_iter_copy, ch.0 + 1, '/') {
                            final_tokens.push(Tokens::COMMENT((line.0, ch.0), None));
                            skip_line = true;
                        } else {
                            final_tokens.push(Tokens::SLASH((line.0, ch.0), None))
                        }
                        final_tokens.push(Tokens::SLASH((line.0, ch.0), None))
                    }
                    ':' => final_tokens.push(Tokens::COLON((line.0, ch.0), None)),
                    '=' => {
                        if next_char_equal(&mut char_iter_copy, ch.0 + 1, '=') {
                            final_tokens.push(Tokens::EQUALS((line.0, ch.0), None));
                            skip = true;
                        } else if next_char_equal(&mut char_iter_copy, ch.0 + 1, '>') {
                            final_tokens.push(Tokens::FARROW((line.0, ch.0), None));
                            skip = true
                        } else {
                            final_tokens.push(Tokens::EQUAL((line.0, ch.0), None))
                        }
                    }
                    '!' => {
                        if next_char_equal(&mut char_iter_copy, ch.0 + 1, '=') {
                            final_tokens.push(Tokens::NEQUAL((line.0, ch.0), None));
                            skip = true
                        } else {
                            final_tokens.push(Tokens::BANG((line.0, ch.0), None))
                        }
                    }
                    '>' => {
                        if next_char_equal(&mut char_iter_copy, ch.0 + 1, '=') {
                            final_tokens.push(Tokens::GEQUAL((line.0, ch.0), None));
                            skip = true
                        } else {
                            final_tokens.push(Tokens::GREATER((line.0, ch.0), None))
                        }
                    }
                    '<' => {
                        if next_char_equal(&mut char_iter_copy, ch.0 + 1, '=') {
                            final_tokens.push(Tokens::LEQUAL((line.0, ch.0), None));
                            skip = true
                        } else {
                            final_tokens.push(Tokens::LESS((line.0, ch.0), None))
                        }
                    }
                    '&' => {
                        if next_char_equal(&mut char_iter_copy, ch.0 + 1, '&') {
                            final_tokens.push(Tokens::AND((line.0, ch.0), None));
                            skip = true
                        }
                    },
                    '|' => {
                        if next_char_equal(&mut char_iter_copy, ch.0 + 1, '|'){
                            final_tokens.push(Tokens::OR((line.0, ch.0), None))
                        } else {
                            final_tokens.push(Tokens::PIPE((line.0, ch.0), None))
                        }
                        skip = true
                    },
                    '"' => {
                        let mut count = 0;
                        loop{
                            count+=1;
                            if next_char_equal(&mut char_iter_copy, ch.0 + count, '"'){
                                break; 
                            }else if next_char_equal(&mut char_iter_copy, ch.0 + count, '\n'){
                                break;
                            }
                        }
                    },
                    '\r' => {},
                    ' ' => {},
                    '\t' => {},
                    _ => syntax_error_print(line, ch, None),
                }
            } else {
                skip = false;
            }
        });
        skip_line = true;
    });
    return final_tokens;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_tokenization() {
        assert_eq!(tokenize("==".to_string()), vec!(Tokens::EQUALS((0,0), None)));
        assert_eq!(
            tokenize("()".to_string()),
            vec!(Tokens::LPAREN((0,0), None), Tokens::RPAREN((0,0), None))
        )
    }
}
