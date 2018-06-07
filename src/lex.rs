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
        ';' | '(' | ')' | '{' | '}' | ',' => {
            cix.next().unwrap();
            Ok(Some(Token::new_punc(ix, ix + 1)))
        }
        _ => Ok(None)
    }
}

pub fn try_op(cix: &mut CharIndices) -> Result<Option<Token>, LexErr> {
    let mut clone = cix.clone();
    let (ix, c) = clone.next().unwrap();
    let (tok, consumed) = match c {
        '*' | '/' | '+' | '-' | '%' => {
            (Token::new_op(ix, ix+1), 1)
        }
        '=' | '>' | '<' => {
            match clone.next() {
                None => return Err(LexErr::Raw(format!("Unexpected EOF at operator `{}`.", c))),
                Some((_, '=')) => (Token::new_op(ix, ix+2), 2),
                _ => (Token::new_op(ix, ix+1), 1)
            }
        }
        '&' => {
            match clone.next() {
                None => return Err(LexErr::Raw(format!("Unexpected EOF at operator `{}`.", c))),
                Some((_, '&')) => (Token::new_op(ix, ix+2), 2),
                _ => return Err(LexErr::Raw("Bitwise & operator is not supported.".to_owned()))
            }
        }
        '|' => {
            match clone.next() {
                None => return Err(LexErr::Raw("Unexpected EOF at operator `{}`.".to_owned())),
                Some((_, '|')) => (Token::new_op(ix, ix+2), 2),
                _ => return Err(LexErr::Raw(format!("Bitwise | operator is not supported."))),
            }
        }
        _ => return Ok(None)
    };
    for _ in 0..consumed { cix.next(); }
    Ok(Some(tok))
}

pub fn try_key(cix: &mut CharIndices) -> Result<Option<Token>, LexErr> {
    if cix.as_str().starts_with("if") {
        match cix.clone().skip(2).next() {
            None => Err(LexErr::Raw(format!("Unexpected EOF after `if`"))),
            Some((end, c)) => {
                if c.is_whitespace() || c == '(' {
                    for _ in 0..2 { cix.next(); } // Consume the 'if'
                    Ok(Some(Token::new_key(end - 2, end)))
                } else { Ok(None) }
            }
        }
    } else if cix.as_str().starts_with("else") {
        match cix.clone().skip(4).next() {
            None => Err(LexErr::Raw(format!("Unexpected EOF after `else`"))),
            Some((end, c)) => {
                if c.is_whitespace() || c == '(' {
                    for _ in 0..4 { cix.next(); } // Consume the 'else'
                    Ok(Some(Token::new_key(end - 4, end)))
                } else { Ok(None) }
            }
        }
    } else if cix.as_str().starts_with("while") {
        match cix.clone().skip(5).next() {
            None => Err(LexErr::Raw(format!("Unexpected EOF after `while`"))),
            Some((end, c)) => {
                if c.is_whitespace() || c == '(' {
                    for _ in 0..5 { cix.next(); } // Consume the 'while'
                    Ok(Some(Token::new_key(end - 5, end)))
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

pub fn try_num_lit(cix: &mut CharIndices) -> Result<Option<Token>, LexErr> {
    let mut clone = cix.clone();
    let (start, first) = clone.next().unwrap();
    if first.is_digit(10) || first == '-' {
        // Consume until we hit a non-digit
        let mut num_consumed = 1;
        let mut consumed_decimal_point = false;
        let mut end = start;
        while let Some((ix, c)) = clone.next() {
            if c.is_alphabetic() {
                return Err(LexErr::Raw("Identifier cannot start with a number".to_owned()));
            } else if c == '.' && consumed_decimal_point {
                return Err(LexErr::Raw("Error: num literal contains more than 1 decimal place".to_owned()));
            } else if !c.is_digit(10) && c != '.' {
                break;
            } else if c == '.' {
                consumed_decimal_point = true;
                continue;
            }
            num_consumed += 1;
            end = ix;
        }
        for _ in 0..num_consumed { cix.next(); } // Advance the iterator
        Ok(Some(Token::new_num_lit(start, end)))
    } else { Ok(None) }
}

pub fn try_core_type(cix: &mut CharIndices) -> Result<Option<Token>, LexErr> {
    let start = cix.clone().next().unwrap().0;
    let as_str = cix.as_str();
    if let Some((tok, to_consume)) = if as_str.starts_with("string") {
        let len = "string".len();
        Some((Token::new_core_type(start, start + len), len))
    } else if as_str.starts_with("float") {
        let len = "float".len();
        Some((Token::new_core_type(start, start + len), len))
    } else if as_str.starts_with("bool") {
        let len = "bool".len();
        Some((Token::new_core_type(start, start + len), len))
    } else if as_str.starts_with("int") {
        let len = "int".len();
        Some((Token::new_core_type(start, start + len), len))
    } else { None } {
        for _ in 0..to_consume { cix.next(); }
        Ok(Some(tok))
    } else {
        Ok(None)
    }
}

pub fn try_bool_lit(cix: &mut CharIndices) -> Result<Option<Token>, LexErr> {
    let start = cix.clone().next().unwrap().0;
    let as_str = cix.as_str();
    if let Some((tok, to_consume)) = if as_str.starts_with("true") {
        let len = "true".len();
        Some((Token::new_bool_lit(start, start + len), len))
    } else if as_str.starts_with("false") {
        let len = "false".len();
        Some((Token::new_bool_lit(start, start + len), len))
    } else { None } {
        for _ in 0..to_consume { cix.next(); }
        Ok(Some(tok))
    } else {
        Ok(None)
    }
}

pub fn try_ident(cix: &mut CharIndices) -> Result<Option<Token>, LexErr> {
    let mut clone = cix.clone();
    let (start, first) = clone.next().unwrap();
    if first.is_alphabetic() {
        // Consume until we hit a non-alphanumeric
        let mut num_consumed = 1;
        let mut end = start + 1;
        while let Some((ix, c)) = clone.next() {
            if !c.is_alphanumeric() { break; }
            num_consumed += 1;
            end = ix + 1;
        }
        for _ in 0..num_consumed { cix.next(); } // Advance the iterator
        Ok(Some(Token::new_ident(start, end)))

    } else { Ok(None) }
}

pub fn lex_token(cix: &mut CharIndices) -> Result<Token, LexErr> {
    if let Some(tok) = try_punc(cix)? {
        Ok(tok)
    } else if let Some(tok) = try_num_lit(cix)? {
        Ok(tok)
    } else if let Some(tok) = try_op(cix)? {
        Ok(tok)
    } else if let Some(tok) = try_bool_lit(cix)? {
        Ok(tok)
    } else if let Some(tok) = try_core_type(cix)? {
        Ok(tok)
    } else if let Some(tok) = try_key(cix)? {
        Ok(tok)
    } else if let Some(tok) = try_string_lit(cix)? {
        Ok(tok)
    } else if let Some(tok) = try_ident(cix)? {
        Ok(tok)
    } else {
        return Err(LexErr::Raw("Unknown token".to_owned()));
    }
}

pub fn lex(src: &str, file: &str) -> Result<Vec<Token>, LexErr> {
    if src.is_empty() {
        return Err(LexErr::Raw("File is empty.".to_owned()));
    }

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
