pub mod binomial;
pub mod chi_square;
pub mod exponential;
pub mod f_dist;
pub mod geometric;
pub mod hypergeometric;
pub mod normal;
pub mod poisson;
pub mod special;
pub mod student_t;
pub mod uniform;

use crate::evaluation::tokenizer::registry::function::operators::FunctionOp;

#[must_use]
pub fn register_distributions() -> Vec<FunctionOp> {
    let mut functions = Vec::new();
    functions.append(&mut binomial::register_binomial());
    functions.append(&mut geometric::register_geometric());
    functions.append(&mut poisson::register_poisson());
    functions.append(&mut normal::register_normal());
    functions.append(&mut student_t::register_student_t());
    functions.append(&mut chi_square::register_chi_square());
    functions.append(&mut f_dist::register_f_dist());
    functions.append(&mut exponential::register_exponential());
    functions.append(&mut hypergeometric::register_hypergeometric());
    functions.append(&mut uniform::register_uniform());
    functions
}
