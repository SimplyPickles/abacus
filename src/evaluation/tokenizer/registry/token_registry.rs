use std::{collections::HashMap, sync::Arc};

use crate::evaluation::tokenizer::registry::{
    binary::{arithmetic::register_arithmetic, operators::BinaryOp},
    function::{
        distributions::register_distributions, math::register_math, operators::FunctionOp,
        stats::register_stats, trig::register_trig,
    },
    unary::{general::register_general, operators::UnaryOp},
};

#[derive(Debug, Default)]
pub struct TokenRegistry {
    pub binary_operators: HashMap<String, Arc<BinaryOp>>,
    pub unary_operators: HashMap<String, Arc<UnaryOp>>,
    pub function_operators: HashMap<String, Arc<FunctionOp>>,

    pub paren_operators: Vec<char>,
}

impl TokenRegistry {
    pub fn new() -> Self {
        Self {
            binary_operators: HashMap::new(),
            unary_operators: HashMap::new(),
            function_operators: HashMap::new(),

            paren_operators: Vec::new(),
        }
    }

    pub fn standard() -> Self {
        let binary = register_arithmetic();

        let mut unary: Vec<UnaryOp> = Vec::new();
        unary.append(&mut register_general());

        let mut functions: Vec<FunctionOp> = Vec::new();
        functions.append(&mut register_trig());
        functions.append(&mut register_math());
        functions.append(&mut register_stats());
        functions.append(&mut register_distributions());

        let paren = vec!['(', ')'];

        Self {
            binary_operators: binary
                .into_iter()
                .map(|op| (op.alias.to_string(), Arc::new(op)))
                .collect(),
            unary_operators: unary
                .into_iter()
                .map(|op| (op.alias.to_string(), Arc::new(op)))
                .collect(),
            function_operators: functions
                .into_iter()
                .map(|op| (op.name.to_string(), Arc::new(op)))
                .collect(),

            paren_operators: paren,
        }
    }
}
