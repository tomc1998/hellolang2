#![allow(dead_code)]

use token::Token;

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
    node_type: NodeType,
    children: Vec<Node>
}

pub fn parse(_tokens: &[Token]) -> Result<Node, ParseErr> {
    unimplemented!();
}
