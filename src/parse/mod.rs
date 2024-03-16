#![allow(dead_code, unused)]
mod printer;
pub use printer::print_node;

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
    IfStatement {
        test: ArenaRef,
        consequent: ArenaRef,
        alternative: Option<ArenaRef>
    },
    ExpressionStatement {
        expression: ArenaRef,
        semicolon: bool
    },
    BlockStatement {
        body: Vec<ArenaRef>
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

fn walk_if_match(iter: &mut TokenIter, token_match: Token) -> Option<()> {
    if let Some(token) = iter.peek() {
        if token.token == token_match {
            iter.next();
            return Some(())
        }
    }
    None
}

pub fn make_tree(
    node_pool: &mut Arena<AstNode>,
    tokens: Vec<PositionalToken>) -> ArenaRef {
    let mut iter = tokens
        .iter()
        .filter(|token| !matches!(token.token, Token::WhiteSpace | Token::LineTerminator))
        .collect::<Vec<_>>();
    let mut iter = iter
        .iter()
        .peekable();
    let mut statements_or_declarations = vec![
        statement(node_pool, &mut iter).unwrap()
    ];
    let mut program = AstNode::Program {
        body: statements_or_declarations 
    };
    node_pool.add(program)
}

fn statement(
    node_pool: &mut Arena<AstNode>,
    iter: &mut TokenIter) -> Option<ArenaRef> {
    if let Some(token) = iter.peek() {
        match &token.token {
            Token::Identifier(identifier) => {
                if identifier == "if" {
                    iter.next();
                    if_statement(node_pool, iter)
                } else {
                    todo!()
                }
            },
            Token::Punctuator(punctuation) => {
                if punctuation == "{" {
                    iter.next();
                    block_statement(node_pool, iter)
                } else if punctuation == "}" {
                    // the statement is empty here?
                    None
                }else {
                    println!("{:?}", punctuation);
                    todo!()
                }
            },
            _ => Some(expression_statement(node_pool, iter))
        }
    } else {
        todo!()
    }
}

fn block_statement(
    node_pool: &mut Arena<AstNode>,
    iter: &mut TokenIter) -> Option<ArenaRef> { 
    let statement = statement(node_pool, iter);
    let mut body = vec![];
    if let Some(statement_ref) = statement {
            walk_if_match(iter, Token::Punctuator(String::from("}")))?;
            body.push(statement_ref);
    }
    Some(node_pool.add(AstNode::BlockStatement { body }))
}

fn if_statement(
    node_pool: &mut Arena<AstNode>,
    iter: &mut TokenIter) -> Option<ArenaRef> { 
    walk_if_match(iter, Token::LeftParen)?;
    let expression = expression(node_pool, iter);
    walk_if_match(iter, Token::RightParen)?;
    let statement = statement(node_pool, iter)?;
    Some(node_pool.add(AstNode::IfStatement {
        test: expression,
        consequent: statement,
        alternative: None
    }))
}

fn expression_statement(
    node_pool: &mut Arena<AstNode>,
    iter: &mut TokenIter) -> ArenaRef {
    let expression = expression(node_pool, iter);
    let mut semicolon = false;
    if let Some(token) = iter.peek() {
        if token.token == Token::Punctuator(String::from(";")) {
            semicolon = true;
        }
    }
    node_pool.add(AstNode::ExpressionStatement {
        expression,
        semicolon
    })
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
            t => { println!("{:?}", t); todo!() }
        };
        return node_pool.add(node);
    }
    todo!()
}
