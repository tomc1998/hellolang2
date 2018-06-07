use token::{Token, TokenType};
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
    Declaration,
    Assignment,
    ParameterList,
    If,
    While,

    Atom,
    FunctionCall,

    // See grammar.bnf for these
    Term0, Term1, Term2, Expression,
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

fn term(tok: Token) -> Node {
    Node {
        node_type: NodeType::Term(tok),
        children: Vec::new(),
    }
}

/// Returns the first token as a terminal given that its value matches the given
/// string. Consumes the token if it matches.
fn assert_term(tokens: &mut TokenIter, src: &str, expected: &str) -> ParseRes {
    let tok = tokens.clone().next();
    match tok {
        Some(tok) => if tok.val(src) == expected {
            Ok(term(*tokens.next().unwrap()))
        } else {
            Err(ParseErr::Point(format!("Expected {}, got {}", expected, tok.val(src)), *tok))
        }
        None => Err(ParseErr::Raw(format!("Expected {}, got EOF", expected))),
    }
}

/// Returns the first token as a terminal given that its value matches the given
/// string. Consumes the token if it matches.
fn assert_term_with_type(tokens: &mut TokenIter, expected: TokenType) -> ParseRes {
    let tok = tokens.clone().next();
    match tok {
        Some(tok) => if tok.token_type == expected {
            Ok(term(*tokens.next().unwrap()))
        } else {
            Err(ParseErr::Point(format!("Expected {:?}", expected), *tok))
        }
        None => Err(ParseErr::Raw(format!("Expected {:?}, got EOF", expected))),
    }
}

/// Returns true if val is an op0 (Mul / div / mod)
fn is_op0(val: &str) -> bool {
    val == "*" || val == "/" || val == "%"
}

/// Returns true if val is an op1 (Addition / subtraction)
fn is_op1(val: &str) -> bool {
    val == "+" || val == "-"
}

/// Returns true if val is an op2 (Comparison)
fn is_op2(val: &str) -> bool {
    val == "==" || val == ">" || val == "<" || val == ">=" || val == "<="
}

/// Returns true if val is an op3 (AND or OR)
fn is_op3(val: &str) -> bool {
    val == "&&" || val == "||"
}

fn parse_function_call(tokens: &mut TokenIter, src: &str) -> ParseRes {
    Ok(Node {
        node_type: NodeType::NTerm(NTermType::FunctionCall),
        children: vec![
            assert_term_with_type(tokens, TokenType::Ident)?,
            assert_term(tokens, src, "(")?,
            parse_parameter_list(tokens, src)?,
            assert_term(tokens, src, ")")?],
    })
}

fn parse_atom(tokens: &mut TokenIter, src: &str) -> ParseRes {
    let mut clone = tokens.clone();
    Ok(Node {
        node_type: NodeType::NTerm(NTermType::Atom),
        children: vec![
            match clone.next().ok_or(ParseErr::Raw("Unexpected EOF".to_owned()))? {
                // Either ident or function call
                tok if tok.token_type == TokenType::Ident => {
                    match clone.next() {
                        // Function call
                        Some(tok) if tok.val(src) == "(" => parse_function_call(tokens, src)?,
                        // Ident
                        _ => term(*tokens.next().unwrap())
                    }
                }
                // Literals
                tok if tok.token_type == TokenType::NumLit => term(*tokens.next().unwrap()),
                tok if tok.token_type == TokenType::StringLit => term(*tokens.next().unwrap()),
                tok if tok.token_type == TokenType::BoolLit => term(*tokens.next().unwrap()),
                tok => return Err(ParseErr::Point("Expected identifier or literal".to_owned(), *tok))
            }]
    })
}

fn parse_term0(tokens: &mut TokenIter, src: &str) -> ParseRes {
    let atom = parse_atom(tokens, src)?;
    Ok(Node {
        node_type: NodeType::NTerm(NTermType::Term0),
        children: match tokens.clone().next() {
            Some(tok) if is_op0(tok.val(src)) =>
                vec![atom, term(*tokens.next().unwrap()), parse_term0(tokens, src)?],
            _ => vec![atom],
        }
    })
}

fn parse_term1(tokens: &mut TokenIter, src: &str) -> ParseRes {
    let term0 = parse_term0(tokens, src)?;
    Ok(Node {
        node_type: NodeType::NTerm(NTermType::Term1),
        children: match tokens.clone().next() {
            Some(tok) if is_op1(tok.val(src)) =>
                vec![term0, term(*tokens.next().unwrap()), parse_term1(tokens, src)?],
            _ => vec![term0],
        }
    })
}

fn parse_term2(tokens: &mut TokenIter, src: &str) -> ParseRes {
    let term1 = parse_term1(tokens, src)?;
    Ok(Node {
        node_type: NodeType::NTerm(NTermType::Term2),
        children: match tokens.clone().next() {
            Some(tok) if is_op2(tok.val(src)) =>
                vec![term1, term(*tokens.next().unwrap()), parse_term2(tokens, src)?],
            _ => vec![term1],
        }
    })
}

fn parse_expression(tokens: &mut TokenIter, src: &str) -> ParseRes {
    let term2 = parse_term2(tokens, src)?;
    Ok(Node {
        node_type: NodeType::NTerm(NTermType::Expression),
        children: match tokens.clone().next() {
            Some(tok) if is_op3(tok.val(src)) =>
                vec![term2, term(*tokens.next().unwrap()), parse_expression(tokens, src)?],
            _ => vec![term2],
        }
    })
}

fn parse_parameter_list(tokens: &mut TokenIter, src: &str) -> ParseRes {
    let mut children = Vec::new();
    loop {
        match tokens.clone().next() {
            Some(tok) if tok.val(src) == ")" => break,
            Some(tok) if tok.val(src) == "," => children.push(term(*tokens.next().unwrap())),
            Some(_) => children.push(parse_expression(tokens, src)?),
            None => return Err(ParseErr::Raw("Unexpected EOF in parameter list".to_owned())),
        }
    }
    Ok(Node {
        node_type: NodeType::NTerm(NTermType::ParameterList),
        children: children,
    })
}

fn parse_if_control(tokens: &mut TokenIter, src: &str) -> ParseRes {
    Ok(Node {
        node_type: NodeType::NTerm(NTermType::If),
        children: vec![
            term(*tokens.next().unwrap()), // if
            assert_term(tokens, src, "(")?,
            parse_expression(tokens, src)?,
            assert_term(tokens, src, ")")?,
            assert_term(tokens, src, "{")?,
            parse_program(tokens, src)?,
            assert_term(tokens, src, "}")?,
            ]
    })
}

fn parse_program(tokens: &mut TokenIter, src: &str) -> ParseRes {
    // Check if this is a control or stmt
    let mut children = Vec::new();
    while let Some(tok) = tokens.clone().next() {
        println!("{:?} - {:?}", tok.val(src), children);
        match tok.val(src) {
            "}" => { break }
            "if" => children.push(parse_if_control(tokens, src)?),
            "while" => unimplemented!(),
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
