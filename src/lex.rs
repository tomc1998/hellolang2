use token::Token;
use std::str::CharIndices;

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum LexErr {
    /// Just an err message
    Raw(String),
    /// Msg, file, line num
    Point(String, String, usize),
}

impl LexErr {
    fn into_point(self, file: String, line_num: usize) -> Self {
        match self {
            LexErr::Raw(s) => LexErr::Point(s, file, line_num),
            _ => panic!("Trying to convert a LexErr::Point into a LexErr::Point!"),
        }
    }

    /// Print this error
    pub fn print_formatted(&self) {
        match *self {
            LexErr::Raw(ref s) => error_raw!("{}", s),
            LexErr::Point(ref s, ref f, ref l) => error_point!(s, f, l),
        }
    }
}

/// Checks if s starts with a punctuation char. All punc is just 1 char in this
/// lang, so just consume 1 char if a punc is found.
pub fn try_punc(cix: &mut CharIndices) -> Result<Option<Token>, LexErr> {
    let (ix, c) = cix.clone().next().unwrap();
    match c {
        ';' | '(' | ')' | '{' | '}' => {
            cix.next().unwrap();
            Ok(Some(Token::new_punc(ix, ix + 1)))
        }
        _ => Ok(None)
    }
}

pub fn try_op(cix: &mut CharIndices) -> Result<Option<Token>, LexErr> {
    let mut clone = cix.clone();
    let (ix, c) = clone.next().unwrap();
    match c {
        '*' | '/' | '+' | '-' | '%' => {
            cix.next().unwrap();
            Ok(Some(Token::new_op(ix, ix+1)))
        }
        '=' | '>' | '<' => {
            match clone.next() {
                None => Err(LexErr::Raw(format!("Unexpected EOF at operator `{}`.", c))),
                Some((_, '=')) => Ok(Some(Token::new_op(ix, ix+2))),
                _ => Ok(Some(Token::new_op(ix, ix+1)))
            }
        }
        '&' => {
            match clone.next() {
                None => Err(LexErr::Raw(format!("Unexpected EOF at operator `{}`.", c))),
                Some((_, '&')) => Ok(Some(Token::new_op(ix, ix+2))),
                _ => Err(LexErr::Raw("Bitwise & operator is not supported.".to_owned()))
            }
        }
        '|' => {
            match clone.next() {
                None => Err(LexErr::Raw("Unexpected EOF at operator `{}`.".to_owned())),
                Some((_, '|')) => Ok(Some(Token::new_op(ix, ix+2))),
                _ => Err(LexErr::Raw(format!("Bitwise | operator is not supported."))),
            }
        }
        _ => Ok(None)
    }
}

pub fn try_key(cix: &mut CharIndices) -> Result<Option<Token>, LexErr> {
    if cix.as_str() == "if" {
        match cix.clone().skip(2).next() {
            None => Err(LexErr::Raw(format!("Unexpected EOF after `if`"))),
            Some((start, c)) => {
                if c.is_whitespace() || c == '(' {
                    for _ in 0..2 { cix.next(); } // Consume the 'if'
                    Ok(Some(Token::new_key(start, start + 2)))
                } else { Ok(None) }
            }
        }
    } else if cix.as_str() == "while" {
        match cix.clone().skip(5).next() {
            None => Err(LexErr::Raw(format!("Unexpected EOF after `while`"))),
            Some((start, c)) => {
                if c.is_whitespace() || c == '(' {
                    for _ in 0..5 { cix.next(); } // Consume the 'while'
                    Ok(Some(Token::new_key(start, start + 5)))
                } else { Ok(None) }
            }
        }
    } else {
        Ok(None)
    }
}

pub fn try_string_lit(cix: &mut CharIndices) -> Result<Option<Token>, LexErr> {
    if cix.clone().next().unwrap().1 == '"' {
        let (start, _) = cix.next().unwrap();
        // Keep consuming until we hit another unescaped "
        let mut escaped = false;
        let mut end = None;
        while let Some((ix, c)) = cix.next() {
            if escaped {
                escaped = false;
                continue;
            } else if c == '\\' {
                escaped = true;
                continue;
            } else if c == '"' {
                end = Some(ix + 1);
                break;
            }
        }
        match end {
            None => Err(LexErr::Raw("Unexpected EOF in string literal".to_owned())),
            Some(end) => Ok(Some(Token::new_string_lit(start, end)))
        }
    } else { Ok(None) }
}

pub fn try_string_lit(cix: &mut CharIndices) -> Result<Option<Token>, LexErr> {
    if cix.clone().next().unwrap().1 == '"' {
        let (start, _) = cix.next().unwrap();
        // Keep consuming until we hit another unescaped "
        let mut escaped = false;
        let mut end = None;
        while let Some((ix, c)) = cix.next() {
            if escaped {
                escaped = false;
                continue;
            } else if c == '\\' {
                escaped = true;
                continue;
            } else if c == '"' {
                end = Some(ix + 1);
                break;
            }
        }
        match end {
            None => Err(LexErr::Raw("Unexpected EOF in string literal".to_owned())),
            Some(end) => Ok(Some(Token::new_string_lit(start, end)))
        }
    } else { Ok(None) }
}

pub fn lex_token(cix: &mut CharIndices) -> Result<Token, LexErr> {
    if let Some(tok) = try_punc(cix)? {
        Ok(tok)
    } else if let Some(tok) = try_op(cix)? {
        Ok(tok)
    } else if let Some(tok) = try_key(cix)? {
        Ok(tok)
    } else if let Some(tok) = try_string_lit(cix)? {
        Ok(tok)
    } else {
        return Err(LexErr::Raw("Unknown token".to_owned()));
    }
}

pub fn lex(src: &str, file: &str) -> Result<Vec<Token>, LexErr> {
    let mut tokens = Vec::new();
    let mut char_ix = src.char_indices();
    let mut line_num = 0;

    while !char_ix.as_str().is_empty() {
        // Check if this is a newline, and increment line_num
        if char_ix.clone().next().unwrap().1 == '\n' {
            char_ix.next();
            line_num += 1;
            continue;
        } else if char_ix.clone().next().unwrap().1.is_whitespace() {
            // Just consume whitespace
            char_ix.next();
            continue;
        }

        // Try lex a token
        match lex_token(&mut char_ix) {
            Ok(tok) => tokens.push(tok),
            Err(e) => return Err(e.into_point(file.to_string(), line_num)),
        }
    }

    Ok(tokens)
}
