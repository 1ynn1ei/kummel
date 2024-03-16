use crate::arena::Arena;
use crate::arena::ArenaRef;
use crate::def::PositionalToken;
use crate::def::Token;
use crate::parse::AstNode;

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
        AstNode::IfStatement { test, consequent, alternative } => {
            let header = ident_string("IfStatement: ", ident);
            let test = print_node(node_pool, test, ident + 2);
            let test_header = ident_string("Test: ", ident + 1);
            let consequent = print_node(node_pool, consequent, ident + 2);
            let consequent_header = ident_string("Consequent: ", ident + 1);
            format!("{header}\n{test_header}\n{test}\n{consequent_header}\n{consequent}")
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
        AstNode::BlockStatement { body } => {
            let header = ident_string("BlockStatement: ", ident);
            format!("{header}")
        },
        _ => todo!(),
    }
}
