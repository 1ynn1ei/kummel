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
    std::slice::Iter<'a, PositionalToken>
    >;

#[derive(Debug)]
pub enum Operator {
    Addition,
    Multiplication,
    Division
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

pub fn make_tree(tokens: Vec<PositionalToken>) -> AstNode {
    let mut iter = tokens.iter().peekable();
    let mut node_pool = Arena::<AstNode>::default();
    let mut last_ref = node_pool.add(AstNode::Program {
        body: Vec::new()
    });
    let expression = expression(&mut node_pool, &mut iter);
    println!("{:?}", node_pool);
    todo!()
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
                iter.next();
                let operator = match punctuation.as_str() {
                    "*" => Operator::Multiplication,
                    "/" => Operator::Division,
                    _ => todo!()
                };
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


pub fn test_expression_evaluator() {
    let mut node_pool = Arena::<AstNode>::default();
    let val_5_idx = node_pool.add(AstNode::Literal { value: 5 });
    let val_8_idx = node_pool.add(AstNode::Literal { value: 8 });
    let val_3_idx = node_pool.add(AstNode::Literal { value: 3 });
    let inner_expression_idx = node_pool.add(
        AstNode::BinaryExpression {
            lhs: val_8_idx,
            rhs: val_3_idx,
            operator: Operator::Division 
        });
   let outer_expression_idx = node_pool.add(
       AstNode::BinaryExpression {
           lhs: val_5_idx,
           rhs: inner_expression_idx,
           operator: Operator::Addition
       });

}
