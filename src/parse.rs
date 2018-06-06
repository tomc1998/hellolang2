#![allow(dead_code)]

use token::Token;
use std;

type TokenIter<'a> = std::slice::Iter<'a, Token>;
type ParseRes = Result<Node, ParseErr>;

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum ParseErr {
    /// Just an err message
    Raw(String),
    /// Err message with token for location
    Point(String, Token),
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum NTermType {
    Program,
    Stmt,
    Control,
    Declaration,
    Assignment,
    FunctionCall,
    Type,
    Identifier,
    Expression,
    ArithmeticExpression,
    StringLiteral,
    BooleanLiteral,
    NumberLiteral,
    Term,
    Op0, Op1,
    ParameterList,
    Parameter,
    If,
    While,
    BooleanExpression,
    BooleanTerm,
    BooleanOperator,
    ComparisonOperator
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum NodeType {
    /// Nonterminal
    NTerm(NTermType),
    /// Terminal
    Term(Token),
}

/// A parse tree node
#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Node {
    /// node_type == Term(_) implies children.len() == 0
    pub node_type: NodeType,
    pub children: Vec<Node>
}

pub fn parse_program(tokens: &mut TokenIter, src: &str) -> ParseRes {
    // Check if this is a control or stmt
    let mut children = Vec::new();
    while let Some(tok) = tokens.clone().next() {
        match tok.val(src) {
            ";" => { tokens.next().unwrap(); }
            "}" => { tokens.next().unwrap(); break }
            "if" | "while" => unimplemented!(),
            _ => unimplemented!(),
        }
    }
    Ok(Node {
        node_type: NodeType::NTerm(NTermType::Program),
        children: children
    })
}

pub fn parse(tokens: &[Token], src: &str) -> ParseRes {
    debug_assert!(!tokens.is_empty());
    parse_program(&mut tokens.iter(), src)
}
