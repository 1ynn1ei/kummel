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
type TokenIter<'s> = std::slice::Iter<'s, PositionalToken<'s>>;
pub enum Operator {
    Addition,
    Division
}

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
        operator: Operator
    }
}

pub fn make_tree(tokens: Vec<PositionalToken>) -> AstNode {
    let mut iter = tokens.iter();
    let mut node_pool = Arena::<AstNode>::default();
    let mut last_ref = node_pool.add(AstNode::Program {
        body: Vec::new()
    });
    let expression = binary_expression(&mut node_pool, &mut iter);
    todo!()
}

fn binary_expression(
    node_pool: &mut Arena<AstNode>,
    iter: &mut TokenIter) -> ArenaRef {
    /*
     * Binary Expression:
     * lhs : Expression
     * rhs : Expression
     * operator
     * expression(lhs, rhs, operator)
     * expression(5, expression(8, 3, /) +)
     */
    if let Some(token) = iter.next() {
        let lhs = match token.token {
            Token::Numeric(number) => AstNode::Literal {
                value: number.to_string().parse::<u64>().unwrap(),
            },
            _ => todo!()
        };

        if let Some(token) = iter.next() {
            let operator = match token.token {
                Token::Punctuator(punctuation) => {
                    match punctuation {
                        "/" => Operator::Division,
                        "+" => Operator::Addition,
                        _ => todo!()
                    }
                },
                _ => todo!()
            };

            if let Some(token) = iter.next() {
                let rhs = match token.token {
                    Token::Numeric(number) => AstNode::Literal {
                        value: number.to_string().parse::<u64>().unwrap(),
                    },
                    _ => todo!()
                };
                let lhs = node_pool.add(lhs);
                let rhs = node_pool.add(rhs);
                return node_pool.add(AstNode::BinaryExpression { lhs, rhs, operator })
            }
        }
    }
    todo!()
}

fn expression(
    node_pool: &mut Arena<AstNode>,
    iter: &mut TokenIter) -> ArenaRef {
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
