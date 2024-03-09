/*
 * we want to be able to turn
 * 5 + 8 / 3
 * into a proper tree.
 */

use crate::arena::Arena;
use crate::arena::ArenaRef;

pub enum Operator {
    Addition,
    Division
}

pub enum AstNode {
    Literal {
        value: u64
    },
    BinaryExpression { 
        lhs: ArenaRef,
        rhs: ArenaRef,
        operator: Operator
    },
    UnaryExpression {
        lhs: ArenaRef,
        operator: Operator
    }
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
