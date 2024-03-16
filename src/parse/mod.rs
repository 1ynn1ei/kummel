#![allow(dead_code, unused)]
/*
 * we want to be able to turn
 * 5 + 8 / 3
 * into a proper tree.
 */

use crate::arena::Arena;
use crate::arena::ArenaRef;
use crate::def::PositionalToken;
use crate::def::Token;
type TokenIter<'a> = 
    std::iter::Peekable<
    std::slice::Iter<'a, &'a PositionalToken>
    >;

#[derive(Debug)]
pub enum Operator {
    Addition,
    Multiplication,
    Division
}

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Operator::Addition => write!(f, "Addition"),
            Operator::Division => write!(f, "Division"),
            _ => todo!()
        }
    }
}

#[derive(Debug)]
pub enum AstNode {
    Program {
        body: Vec<ArenaRef>
    },
    Literal {
        value: u64
    },
    Grouping {
        expression: ArenaRef
    },
    BinaryExpression { 
        lhs: ArenaRef,
        rhs: ArenaRef,
        operator: Operator
    },
    UnaryExpression {
        expression: ArenaRef,
    }
}


fn ident_string(old_string: &str, ident: usize) -> String {
    let mut string = String::new();
    for i in 0..ident {
        string.push(' ');
        string.push(' ');
    }
    format!("{string}{old_string}")
}

pub fn print_node(
    node_pool: &Arena<AstNode>,
    cur_ref: &ArenaRef,
    ident: usize,
    ) -> String {
    let node = node_pool.get(*cur_ref).unwrap();
    match node {
        AstNode::Program { body } => {
            let header = ident_string("Program: ", ident);
            let body_str = body
                .iter()
                .fold("".to_string(), |acc, next| {
                    acc + &print_node(node_pool, next, ident + 1) + "\n"
                });
            format!("{header}\n{body_str}")
        },
        AstNode::Literal { value } => {
            let header = ident_string("Literal: ", ident);
            let value_str = ident_string(
                value.to_string().as_str(),
                ident + 1);
            format!("{header}\n{value_str}")
        },
        AstNode::BinaryExpression { lhs, rhs, operator } => {
            let header = ident_string("BinaryExpression: ",ident);
            let lhs = print_node(node_pool, lhs, ident + 1);
            let rhs = print_node(node_pool, rhs, ident + 1);
            let operator_str = ident_string("Operator: ", ident + 1);

            format!("{header}\n{lhs}\n{rhs}\n{operator_str}{operator}")
        },
        AstNode::UnaryExpression { expression } => {
            let header = ident_string("UnaryExpression: ",ident);
            let expression = print_node(node_pool, expression, ident + 1);
            format!("{header}\n{expression}")
        },
        _ => todo!()
    }
}

pub fn make_tree(
    node_pool: &mut Arena<AstNode>,
    tokens: Vec<PositionalToken>) -> ArenaRef {
    let mut iter = tokens
        .iter()
        .filter(|token| !matches!(token.token, Token::WhiteSpace))
        .collect::<Vec<_>>();
    let mut iter = iter
        .iter()
        .peekable();
    let mut expressions_or_declarations = vec![
        expression(node_pool, &mut iter)
    ];
    let mut program = AstNode::Program {
        body: expressions_or_declarations
    };
    node_pool.add(program)
}

fn expression(
    node_pool: &mut Arena<AstNode>,
    iter: &mut TokenIter) -> ArenaRef {
    additive_expression(node_pool, iter)
}

fn additive_expression(
    node_pool: &mut Arena<AstNode>,
    iter: &mut TokenIter) -> ArenaRef {
    let lhs = multiplicative_expression(node_pool, iter);
    if let Some(token) = iter.peek() {
        match &token.token {
            Token::Punctuator(punctuation) => {
                iter.next();
                let operator = match punctuation.as_str() {
                    "+" => Operator::Addition,
                    _ => todo!()
                };
                let rhs = additive_expression(node_pool, iter);
                node_pool.add(AstNode::BinaryExpression {
                    lhs,
                    rhs,
                    operator
                })
            },
            _ => lhs
        }
    } else {
        lhs
    }
}

fn multiplicative_expression(
    node_pool: &mut Arena<AstNode>,
    iter: &mut TokenIter) -> ArenaRef {
    let lhs = unary_expression(node_pool, iter);
    if let Some(token) = iter.peek() {
        match &token.token {
            Token::Punctuator(punctuation) => {
                let operator = match punctuation.as_str() {
                    "*" => Operator::Multiplication,
                    "/" => Operator::Division,
                    p => { return lhs }
                };
                iter.next();
                let rhs = multiplicative_expression(node_pool, iter);
                node_pool.add(AstNode::BinaryExpression {
                    lhs,
                    rhs,
                    operator
                })
            },
            _ => lhs
        }
    } else {
        lhs
    }
}

fn unary_expression(
    node_pool: &mut Arena<AstNode>,
    iter: &mut TokenIter) -> ArenaRef {
    let literal = literal(node_pool, iter);
    node_pool.add(AstNode::UnaryExpression { expression: literal })
}

fn literal(
    node_pool: &mut Arena<AstNode>,
    iter: &mut TokenIter) -> ArenaRef {
    if let Some(token) = iter.next() {
        let node = match &token.token {
            Token::Numeric(number) => AstNode::Literal {
                value: number.to_string().parse::<u64>().unwrap(),
            },
            _ => todo!()
        };
        return node_pool.add(node);
    }
    todo!()
}
