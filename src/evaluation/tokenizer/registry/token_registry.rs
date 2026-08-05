use std::{collections::HashMap, sync::Arc};

use crate::evaluation::tokenizer::registry::{
    binary::{arithmetic::register_arithmetic, operators::BinaryOp},
    function::{
        combinatorics::register_combinatorics, distributions::register_distributions,
        financial::register_financial, math::register_math, math_helpers::register_math_helpers,
        operators::FunctionOp, stats::register_stats, trig::register_trig,
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

            paren_operators: vec!['(', ')'],
        }
    }

    pub fn standard() -> Self {
        let binary = register_arithmetic();

        let mut unary: Vec<UnaryOp> = Vec::new();
        unary.append(&mut register_general());

        let mut functions: Vec<FunctionOp> = Vec::new();
        functions.append(&mut register_trig());
        functions.append(&mut register_math());
        functions.append(&mut register_combinatorics());
        functions.append(&mut register_math_helpers());
        functions.append(&mut register_financial());
        functions.append(&mut register_stats());
        functions.append(&mut register_distributions());
        use crate::evaluation::tokenizer::registry::function::ci::register_ci;
        functions.append(&mut register_ci());

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

            paren_operators: vec!['(', ')'],
        }
    }

    pub fn register_binary_operator(&mut self, alias: &str, op: BinaryOp) {
        self.binary_operators
            .insert(alias.to_string(), Arc::new(op));
    }

    pub fn register_unary_operator(&mut self, alias: &str, op: UnaryOp) {
        self.unary_operators.insert(alias.to_string(), Arc::new(op));
    }

    pub fn register_function_operator(&mut self, alias: &str, op: FunctionOp) {
        self.function_operators
            .insert(alias.to_string(), Arc::new(op));
    }
}
