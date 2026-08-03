use std::{collections::HashMap, sync::Arc};

use crate::evaluation::tokenizer::registry::{
    binary::{arithmetic::register_arithmetic, operators::BinaryOp},
    unary::{general::register_general, operators::UnaryOp},
};

#[derive(Debug, Default)]
pub struct TokenRegistry {
    pub binary_operators: HashMap<String, Arc<BinaryOp>>,
    pub unary_operators: HashMap<String, Arc<UnaryOp>>,

    pub paren_operators: Vec<char>,
}

impl TokenRegistry {
    pub fn new() -> Self {
        Self {
            binary_operators: HashMap::new(),
            unary_operators: HashMap::new(),

            paren_operators: Vec::new(),
        }
    }

    pub fn standard() -> Self {
        let binary = register_arithmetic();

        let mut unary: Vec<UnaryOp> = Vec::new();
        unary.append(&mut register_general());

        let mut paren: Vec<char> = Vec::new();

        paren.push('(');
        paren.push(')');

        Self {
            binary_operators: binary
                .into_iter()
                .map(|op| (op.alias.to_string(), Arc::new(op)))
                .collect(),
            unary_operators: unary
                .into_iter()
                .map(|op| (op.alias.to_string(), Arc::new(op)))
                .collect(),

            paren_operators: paren,
        }
    }
}
