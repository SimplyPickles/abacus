use crate::{
    AbacusError, Value,
    evaluation::tokenizer::registry::function::{
        distributions::{
            chi_square::compute_chisqcdf, normal::std_normal_cdf, special::make_dimensionless,
            student_t::compute_tcdf,
        },
        operators::{FunctionOp, FunctionTarget},
    },
    units::{eval_result::EvalResult, hash::Hash, value::Value as AbacusValue},
};
use std::sync::Arc;

// ── 1. ZTest (ZTest, ztest, Z_Test) ──
// Forms:
// ZTest(mu0, xbar, sigma, n) -> summary stats
// ZTest(mu0, sigma, data...) -> sample data list
fn z_test_fn(args: &[Value]) -> Result<EvalResult, AbacusError> {
    if args.len() < 3 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    let mu0 = &args[0];
    let (xbar, sigma, n) = if args.len() == 4 && args[1].unit.is_compatible_with(&mu0.unit) {
        // Summary stats mode: (mu0, xbar, sigma, n)
        let xbar = &args[1];
        let sigma = &args[2];
        let n = args[3].canonical;

        if !sigma.unit.is_compatible_with(&mu0.unit) || n <= 0.0 {
            return Err(AbacusError::IncompatibleFunctionArguments);
        }
        (xbar.canonical, sigma.canonical, n)
    } else {
        // Data list mode: (mu0, sigma, data...)
        let sigma = &args[1];
        if !sigma.unit.is_compatible_with(&mu0.unit) {
            return Err(AbacusError::IncompatibleFunctionArguments);
        }
        let data = &args[2..];
        for d in data {
            if !d.unit.is_compatible_with(&mu0.unit) {
                return Err(AbacusError::IncompatibleDimensions);
            }
        }
        let n = data.len() as f64;
        let sum: f64 = data.iter().map(|v| v.canonical).sum();
        let mean = sum / n;
        (mean, sigma.canonical, n)
    };

    let se = sigma / n.sqrt();
    if se <= 0.0 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    let z = (xbar - mu0.canonical) / se;
    let p_value = 2.0 * (1.0 - std_normal_cdf(z.abs()));

    let mut hash = Hash::new();
    hash.insert("z", make_dimensionless(z));
    hash.insert("p", make_dimensionless(p_value));
    hash.insert("p_value", make_dimensionless(p_value));
    hash.insert(
        "mean",
        AbacusValue {
            canonical: xbar,
            unit: Arc::clone(&mu0.unit),
        },
    );
    hash.insert("n", make_dimensionless(n));

    Ok(EvalResult::Hash(hash))
}

// ── 2. TTest (TTest, ttest, T_Test) ──
// Forms:
// TTest(mu0, xbar, s, n) -> summary stats
// TTest(mu0, data...) -> sample data list
fn t_test_fn(args: &[Value]) -> Result<EvalResult, AbacusError> {
    if args.len() < 2 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    let mu0 = &args[0];
    let (xbar, s, n) = if args.len() == 4 && args[1].unit.is_compatible_with(&mu0.unit) {
        // Summary stats mode: (mu0, xbar, s, n)
        let xbar = &args[1];
        let s = &args[2];
        let n = args[3].canonical;

        if !s.unit.is_compatible_with(&mu0.unit) || n <= 1.0 {
            return Err(AbacusError::IncompatibleFunctionArguments);
        }
        (xbar.canonical, s.canonical, n)
    } else {
        // Data list mode: (mu0, data...)
        let data = &args[1..];
        for d in data {
            if !d.unit.is_compatible_with(&mu0.unit) {
                return Err(AbacusError::IncompatibleDimensions);
            }
        }
        let n = data.len() as f64;
        if n <= 1.0 {
            return Err(AbacusError::IncompatibleFunctionArguments);
        }
        let mean = data.iter().map(|v| v.canonical).sum::<f64>() / n;
        let var = data
            .iter()
            .map(|v| (v.canonical - mean).powi(2))
            .sum::<f64>()
            / (n - 1.0);
        let s = var.sqrt();
        (mean, s, n)
    };

    let se = s / n.sqrt();
    if se <= 0.0 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    let t = (xbar - mu0.canonical) / se;
    let df = n - 1.0;
    let p_value = 2.0 * (1.0 - compute_tcdf(df, t.abs()));

    let mut hash = Hash::new();
    hash.insert("t", make_dimensionless(t));
    hash.insert("p", make_dimensionless(p_value));
    hash.insert("p_value", make_dimensionless(p_value));
    hash.insert("df", make_dimensionless(df));
    hash.insert(
        "mean",
        AbacusValue {
            canonical: xbar,
            unit: Arc::clone(&mu0.unit),
        },
    );
    hash.insert(
        "s",
        AbacusValue {
            canonical: s,
            unit: Arc::clone(&mu0.unit),
        },
    );
    hash.insert("n", make_dimensionless(n));

    Ok(EvalResult::Hash(hash))
}

// ── 3. 1-PropZTest (1-PropZTest, 1_PropZTest, 1PropZTest, propztest) ──
// Form: 1-PropZTest(p0, x, n)
fn one_prop_z_test_fn(args: &[Value]) -> Result<EvalResult, AbacusError> {
    if args.len() != 3 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    let p0 = args[0].canonical;
    let x = args[1].canonical;
    let n = args[2].canonical;

    if p0 <= 0.0 || p0 >= 1.0 || n <= 0.0 || x < 0.0 || x > n {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    let phat = x / n;
    let se = (p0 * (1.0 - p0) / n).sqrt();
    let z = (phat - p0) / se;
    let p_value = 2.0 * (1.0 - std_normal_cdf(z.abs()));

    let mut hash = Hash::new();
    hash.insert("z", make_dimensionless(z));
    hash.insert("p", make_dimensionless(p_value));
    hash.insert("p_value", make_dimensionless(p_value));
    hash.insert("phat", make_dimensionless(phat));
    hash.insert("n", make_dimensionless(n));

    Ok(EvalResult::Hash(hash))
}

// ── 4. 2-SampZTest (2-SampZTest, 2_SampZTest, 2SampZTest, sampztest2) ──
// Form: 2-SampZTest(sigma1, sigma2, xbar1, n1, xbar2, n2)
fn two_samp_z_test_fn(args: &[Value]) -> Result<EvalResult, AbacusError> {
    if args.len() != 6 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    let s1 = &args[0];
    let s2 = &args[1];
    let x1 = &args[2];
    let n1 = args[3].canonical;
    let x2 = &args[4];
    let n2 = args[5].canonical;

    if !s2.unit.is_compatible_with(&s1.unit)
        || !x1.unit.is_compatible_with(&s1.unit)
        || !x2.unit.is_compatible_with(&s1.unit)
        || n1 <= 0.0
        || n2 <= 0.0
    {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    let var1 = s1.canonical * s1.canonical;
    let var2 = s2.canonical * s2.canonical;
    let se = (var1 / n1 + var2 / n2).sqrt();
    let diff = x1.canonical - x2.canonical;
    let z = diff / se;
    let p_value = 2.0 * (1.0 - std_normal_cdf(z.abs()));

    let mut hash = Hash::new();
    hash.insert("z", make_dimensionless(z));
    hash.insert("p", make_dimensionless(p_value));
    hash.insert("p_value", make_dimensionless(p_value));
    hash.insert(
        "diff",
        AbacusValue {
            canonical: diff,
            unit: Arc::clone(&s1.unit),
        },
    );

    Ok(EvalResult::Hash(hash))
}

// ── 5. 2-SampTTest (2-SampTTest, 2_SampTTest, 2SampTTest, sampttest2) ──
// Form: 2-SampTTest(xbar1, s1, n1, xbar2, s2, n2)
fn two_samp_t_test_fn(args: &[Value]) -> Result<EvalResult, AbacusError> {
    if args.len() != 6 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    let x1 = &args[0];
    let s1 = &args[1];
    let n1 = args[2].canonical;
    let x2 = &args[3];
    let s2 = &args[4];
    let n2 = args[5].canonical;

    if !s1.unit.is_compatible_with(&x1.unit)
        || !x2.unit.is_compatible_with(&x1.unit)
        || !s2.unit.is_compatible_with(&x1.unit)
        || n1 <= 1.0
        || n2 <= 1.0
    {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    let v1 = (s1.canonical * s1.canonical) / n1;
    let v2 = (s2.canonical * s2.canonical) / n2;
    let se = (v1 + v2).sqrt();
    let diff = x1.canonical - x2.canonical;
    let t = diff / se;

    // Welch-Satterthwaite degrees of freedom
    let num = (v1 + v2) * (v1 + v2);
    let den = (v1 * v1) / (n1 - 1.0) + (v2 * v2) / (n2 - 1.0);
    let df = num / den;

    let p_value = 2.0 * (1.0 - compute_tcdf(df, t.abs()));

    let mut hash = Hash::new();
    hash.insert("t", make_dimensionless(t));
    hash.insert("p", make_dimensionless(p_value));
    hash.insert("p_value", make_dimensionless(p_value));
    hash.insert("df", make_dimensionless(df));
    hash.insert(
        "diff",
        AbacusValue {
            canonical: diff,
            unit: Arc::clone(&x1.unit),
        },
    );

    Ok(EvalResult::Hash(hash))
}

// ── 6. 2-PropZTest (2-PropZTest, 2_PropZTest, 2PropZTest, propztest2) ──
// Form: 2-PropZTest(x1, n1, x2, n2)
fn two_prop_z_test_fn(args: &[Value]) -> Result<EvalResult, AbacusError> {
    if args.len() != 4 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    let x1 = args[0].canonical;
    let n1 = args[1].canonical;
    let x2 = args[2].canonical;
    let n2 = args[3].canonical;

    if n1 <= 0.0 || n2 <= 0.0 || x1 < 0.0 || x1 > n1 || x2 < 0.0 || x2 > n2 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    let p1 = x1 / n1;
    let p2 = x2 / n2;
    let p_pooled = (x1 + x2) / (n1 + n2);
    let se = (p_pooled * (1.0 - p_pooled) * (1.0 / n1 + 1.0 / n2)).sqrt();
    let diff = p1 - p2;
    let z = diff / se;
    let p_value = 2.0 * (1.0 - std_normal_cdf(z.abs()));

    let mut hash = Hash::new();
    hash.insert("z", make_dimensionless(z));
    hash.insert("p", make_dimensionless(p_value));
    hash.insert("p_value", make_dimensionless(p_value));
    hash.insert("p1", make_dimensionless(p1));
    hash.insert("p2", make_dimensionless(p2));
    hash.insert("diff", make_dimensionless(diff));

    Ok(EvalResult::Hash(hash))
}

// ── 7. Chi2Test (Chi2Test, chi2test, Chi2_Test) ──
// Forms:
// Chi2Test(obs1, exp1, obs2, exp2, ...) -> paired args
fn chi2_test_fn(args: &[Value]) -> Result<EvalResult, AbacusError> {
    if args.len() < 4 || !args.len().is_multiple_of(2) {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    let k = args.len() / 2;
    let obs_slice = &args[..k];
    let exp_slice = &args[k..];

    let mut chi2 = 0.0;
    for (o, e) in obs_slice.iter().zip(exp_slice.iter()) {
        let e_val = e.canonical;
        if e_val <= 0.0 {
            return Err(AbacusError::IncompatibleFunctionArguments);
        }
        let diff = o.canonical - e_val;
        chi2 += (diff * diff) / e_val;
    }

    let df = (k - 1) as f64;
    let p_value = 1.0 - compute_chisqcdf(df, chi2);

    let mut hash = Hash::new();
    hash.insert("chi2", make_dimensionless(chi2));
    hash.insert("p", make_dimensionless(p_value));
    hash.insert("p_value", make_dimensionless(p_value));
    hash.insert("df", make_dimensionless(df));

    Ok(EvalResult::Hash(hash))
}

pub fn register_hypothesis() -> Vec<FunctionOp> {
    let mut ops = Vec::new();

    // ZTest
    for name in &["ZTest", "ztest", "Z_Test"] {
        ops.push(FunctionOp {
            name,
            min_args: 3,
            max_args: usize::MAX,
            func: FunctionTarget::EvalResult(z_test_fn),
        });
    }

    // TTest
    for name in &["TTest", "ttest", "T_Test"] {
        ops.push(FunctionOp {
            name,
            min_args: 2,
            max_args: usize::MAX,
            func: FunctionTarget::EvalResult(t_test_fn),
        });
    }

    // 1-PropZTest
    for name in &["1-PropZTest", "1_PropZTest", "1PropZTest", "propztest"] {
        ops.push(FunctionOp {
            name,
            min_args: 3,
            max_args: 3,
            func: FunctionTarget::EvalResult(one_prop_z_test_fn),
        });
    }

    // 2-SampZTest
    for name in &["2-SampZTest", "2_SampZTest", "2SampZTest", "sampztest2"] {
        ops.push(FunctionOp {
            name,
            min_args: 6,
            max_args: 6,
            func: FunctionTarget::EvalResult(two_samp_z_test_fn),
        });
    }

    // 2-SampTTest
    for name in &["2-SampTTest", "2_SampTTest", "2SampTTest", "sampttest2"] {
        ops.push(FunctionOp {
            name,
            min_args: 6,
            max_args: 6,
            func: FunctionTarget::EvalResult(two_samp_t_test_fn),
        });
    }

    // 2-PropZTest
    for name in &["2-PropZTest", "2_PropZTest", "2PropZTest", "propztest2"] {
        ops.push(FunctionOp {
            name,
            min_args: 4,
            max_args: 4,
            func: FunctionTarget::EvalResult(two_prop_z_test_fn),
        });
    }

    // Chi2Test
    for name in &["Chi2Test", "chi2test", "Chi2_Test"] {
        ops.push(FunctionOp {
            name,
            min_args: 4,
            max_args: usize::MAX,
            func: FunctionTarget::EvalResult(chi2_test_fn),
        });
    }

    ops
}
