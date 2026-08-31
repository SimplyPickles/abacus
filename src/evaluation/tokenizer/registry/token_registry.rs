use std::{collections::HashMap, sync::Arc};

use crate::evaluation::tokenizer::registry::{
    binary::{arithmetic::register_arithmetic, operators::BinaryOp},
    function::{
        ci::register_ci, combinatorics::register_combinatorics, date::register_date_functions,
        distributions::register_distributions, financial::register_financial,
        hypothesis::register_hypothesis, math::register_math, math_helpers::register_math_helpers,
        operators::FunctionOp, regression::register_regression, stats::register_stats,
        trig::register_trig,
    },
    unary::{general::register_general, operators::UnaryOp},
};

#[derive(Debug, Default)]
pub struct TokenRegistry {
    pub binary_operators: HashMap<String, Arc<BinaryOp>>,
    pub unary_operators: HashMap<String, Arc<UnaryOp>>,
    pub function_operators: HashMap<String, Arc<FunctionOp>>,
}

impl TokenRegistry {
    pub fn new() -> Self {
        Self {
            binary_operators: HashMap::new(),
            unary_operators: HashMap::new(),
            function_operators: HashMap::new(),
        }
    }

    pub fn standard() -> Self {
        let binary = register_arithmetic();

        let mut unary: Vec<UnaryOp> = Vec::new();
        unary.append(&mut register_general());

        let functions = [
            register_trig(),
            register_math(),
            register_combinatorics(),
            register_math_helpers(),
            register_financial(),
            register_stats(),
            register_distributions(),
            register_ci(),
            register_regression(),
            register_hypothesis(),
            register_date_functions(),
        ]
        .concat();

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
