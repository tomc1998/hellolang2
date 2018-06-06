/// An index into some source code
#[derive(Ord, Eq, PartialEq, PartialOrd, Debug, Clone, Copy, Hash)]
pub struct Point(usize);

#[derive(Eq, PartialEq, Debug, Clone, Copy, Hash)]
pub enum TokenType {
    Ident, Punc, Key, Op, NumLit, StringLit, BoolLit, CoreType,
}

#[derive(Eq, PartialEq, Debug, Clone, Copy, Hash)]
pub struct Token {
    start: Point,
    end: Point,
    token_type: TokenType,
}

impl<'a> Token {
    pub fn new_ident(start: usize, end: usize) -> Token {
        Token { start: Point(start), end: Point(end), token_type: TokenType::Ident }
    }
    pub fn new_punc(start: usize, end: usize) -> Token {
        Token { start: Point(start), end: Point(end), token_type: TokenType::Punc }
    }
    pub fn new_key(start: usize, end: usize) -> Token {
        Token { start: Point(start), end: Point(end), token_type: TokenType::Key }
    }
    pub fn new_op(start: usize, end: usize) -> Token {
        Token { start: Point(start), end: Point(end), token_type: TokenType::Op }
    }
    pub fn new_num_lit(start: usize, end: usize) -> Token {
        Token { start: Point(start), end: Point(end), token_type: TokenType::NumLit }
    }
    pub fn new_string_lit(start: usize, end: usize) -> Token {
        Token { start: Point(start), end: Point(end), token_type: TokenType::StringLit }
    }
    pub fn new_bool_lit(start: usize, end: usize) -> Token {
        Token { start: Point(start), end: Point(end), token_type: TokenType::BoolLit }
    }
    pub fn new_core_type(start: usize, end: usize) -> Token {
        Token { start: Point(start), end: Point(end), token_type: TokenType::CoreType }
    }

    pub fn val(&'a self, src: &'a str) -> &'a str {
        unsafe {
            src.slice_unchecked(self.start.0, self.end.0)
        }
    }
}
