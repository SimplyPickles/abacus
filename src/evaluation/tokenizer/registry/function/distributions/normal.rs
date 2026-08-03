use crate::{
    evaluation::tokenizer::registry::function::{
        distributions::special::make_dimensionless, operators::FunctionOp,
    },
    AbacusError, Value,
};
use std::f64::consts::TAU;

fn erf(x: f64) -> f64 {
    let a1 = 0.254829592;
    let a2 = -0.284496736;
    let a3 = 1.421413741;
    let a4 = -1.453152027;
    let a5 = 1.061405429;
    let p = 0.3275911;

    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let x = x.abs();

    let t = 1.0 / (1.0 + p * x);
    let y = 1.0 - (((((a5 * t + a4) * t) + a3) * t + a2) * t + a1) * t * (-x * x).exp();

    sign * y
}

fn parse_norm_args(args: &[Value]) -> Result<(f64, f64, f64), AbacusError> {
    if args.len() == 1 {
        if !args[0].unit.is_dimensionless() {
            return Err(AbacusError::IncompatibleDimensions);
        }
        Ok((args[0].canonical, 0.0, 1.0))
    } else if args.len() == 3 {
        let first_unit = &args[0].unit;
        if !args[1].unit.is_compatible_with(first_unit)
            || !args[2].unit.is_compatible_with(first_unit)
        {
            return Err(AbacusError::IncompatibleDimensions);
        }
        let x = args[0].canonical;
        let mean = args[1].canonical;
        let std = args[2].canonical;
        if std <= 0.0 {
            return Err(AbacusError::IncompatibleFunctionArguments);
        }
        Ok((x, mean, std))
    } else {
        Err(AbacusError::IncompatibleFunctionArguments)
    }
}

/// normpdf(x) or normpdf(x, mean, std)
fn normpdf_fn(args: &[Value]) -> Result<Value, AbacusError> {
    let (x, mean, std) = parse_norm_args(args)?;
    let z = (x - mean) / std;
    let pdf = (1.0 / (std * TAU.sqrt())) * (-0.5 * z * z).exp();
    Ok(make_dimensionless(pdf))
}

/// normcdf(x) or normcdf(x, mean, std)
fn normcdf_fn(args: &[Value]) -> Result<Value, AbacusError> {
    let (x, mean, std) = parse_norm_args(args)?;
    let z = (x - mean) / std;
    let cdf = 0.5 * (1.0 + erf(z / std::f64::consts::SQRT_2));
    Ok(make_dimensionless(cdf))
}

pub fn register_normal() -> Vec<FunctionOp> {
    vec![
        FunctionOp {
            name: "normpdf",
            min_args: 1,
            max_args: 3,
            func: normpdf_fn,
        },
        FunctionOp {
            name: "normcdf",
            min_args: 1,
            max_args: 3,
            func: normcdf_fn,
        },
    ]
}
