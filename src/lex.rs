use token::Token;
use std::str::CharIndices;

/// Checks if s starts with a punctuation char. All punc is just 1 char in this
/// lang, so just consume 1 char if a punc is found.
pub fn try_punc(cix: &mut CharIndices, _file: &str) -> Option<Token> {
    let (ix, c) = cix.clone().next().unwrap();
    match c {
        ';' | '(' | ')' | '{' | '}' => {
            cix.next().unwrap();
            Some(Token::new_punc(ix, ix + 1))
        }
        _ => None
    }
}

pub fn try_op(cix: &mut CharIndices, file: &str) -> Option<Token> {
    let mut clone = cix.clone();
    let (ix, c) = clone.next().unwrap();
    match c {
        '*' | '/' | '+' | '-' | '%' => {
            cix.next().unwrap();
            Some(Token::new_op(ix, ix+1))
        }
        '=' | '>' | '<' => {
            match clone.next() {
                None => {
                    error_raw!("Unexpected EOF at operator `{}`.", c);
                    None
                }
                Some((_, '=')) => Some(Token::new_op(ix, ix+2)),
                _ => Some(Token::new_op(ix, ix+1))
            }
        }
        '&' => {
            match clone.next() {
                None => {
                    error_raw!("Unexpected EOF at operator `{}`.", c);
                    None
                }
                Some((_, '&')) => Some(Token::new_op(ix, ix+2)),
                _ => {
                    error_point!("Bitwise & operator is not supported.", file, ix);
                    None
                }
            }
        }
        '|' => {
            match clone.next() {
                None => {
                    error_raw!("Unexpected EOF at operator `{}`.", c);
                    None
                }
                Some((_, '|')) => Some(Token::new_op(ix, ix+2)),
                _ => {
                    error_point!("Bitwise | operator is not supported.", file, ix);
                    None
                }
            }
        }
        _ => None
    }
}

pub fn try_key(cix: &mut CharIndices, _file: &str) -> Option<Token> {
    if cix.as_str() == "if" {
        match cix.clone().skip(2).next() {
            None => {
                error_raw!("Unexpected EOF after `if`");
                None
            }
            Some((start, c)) => {
                if c.is_whitespace() || c == '(' {
                    for _ in 0..2 { cix.next(); } // Consume the 'if'
                    Some(Token::new_key(start, start + 2))
                } else { None }
            }
        }
    } else if cix.as_str() == "while" {
        match cix.clone().skip(5).next() {
            None => {
                error_raw!("Unexpected EOF after `while`");
                None
            }
            Some((start, c)) => {
                if c.is_whitespace() || c == '(' {
                    for _ in 0..5 { cix.next(); } // Consume the 'while'
                    Some(Token::new_key(start, start + 5))
                } else { None }
            }
        }
    } else {
        None
    }
}

pub fn lex(src: &str, file: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut char_ix = src.char_indices();

    while !char_ix.as_str().is_empty() {
        // Consume whitespace
        if char_ix.clone().next().unwrap().1.is_whitespace() {
            char_ix.next();
            continue;
        }

        if let Some(tok) = try_punc(&mut char_ix, file) {
            tokens.push(tok);
        } else if let Some(tok) = try_op(&mut char_ix, file) {
            tokens.push(tok);
        } else if let Some(tok) = try_key(&mut char_ix, file) {
            tokens.push(tok);
        } else {
            error_point!("Unknown token", file, char_ix.next().unwrap().0);
            break;
        }
    }

    tokens
}
