use abacus::{
    AbacusError, UnitRegistry, evaluation::parser::parse::evaluate,
    evaluation::tokenizer::registry::token_registry::TokenRegistry,
};

fn main() -> Result<(), AbacusError> {
    let token_reg = TokenRegistry::standard();
    let unit_reg = UnitRegistry::standard();

    let expressions = [
        "// Basic Arithmetic & Units",
        "5 m + 3 m",
        "(5 m + 20 cm) as m",
        "2 + 3 * 4",
        "5(2 + 3)",
        "(2 + 3)(4 + 5)",
        "2 sqrt(9 m^2)",
        "2(10 m)",
        "1m/m",
        "1 m/m in inches",
        "5 km / m",
        "5 km to m",
        "",
        "// Statistical Functions & Ranges",
        "sum(1 m .. 5 m)",
        "mean(10 m .. 30 m)",
        "median(1 m, 10 m, 5 m, 20 m)",
        "mode(2 m, 5 m, 2 m, 8 m)",
        "range(1 m .. 10 m)",
        "quantile(1 m .. 5 m, 0.75)",
        "percentile(1 m .. 5 m, 75)",
        "iqr(1 m .. 5 m)",
        "corr(1..5, 2..6)",
        "var(1 m .. 5 m)",
        "std(1 m .. 5 m)",
        "",
        "// Discrete Distributions",
        "binompdf(10, 0.5, 5)",
        "binomcdf(10, 0.5, 5)",
        "geompdf(0.5, 3)",
        "geomcdf(0.5, 3)",
        "poissonpdf(3, 2)",
        "poissoncdf(3, 2)",
        "hypgeompdf(20, 7, 12, 4)",
        "",
        "// Continuous Distributions & Inverse CDFs",
        "normpdf(0)",
        "normcdf(0)",
        "normcdf(70 kg, 65 kg, 5 kg)",
        "invnorm(0.975)",
        "invnorm(0.975, 100 kg, 15 kg)",
        "tcdf(10, 2.228)",
        "invt(0.975, 10)",
        "chisqcdf(10, 18.307)",
        "invchisq(0.95, 10)",
        "fcdf(5, 10, 3.33)",
        "expcdf(0.5, 2)",
        "invexp(0.63212, 0.5)",
        "unifcdf(0, 10, 5)",
        "invunif(0.5, 0, 10)",
    ];

    println!("=== Abacus Math Engine Demonstration ===\n");

    for expr in expressions {
        if expr.is_empty() {
            println!();
            continue;
        }
        if expr.starts_with("//") {
            println!("\x1b[1;36m{expr}\x1b[0m");
            continue;
        }

        match evaluate(&token_reg, &unit_reg, expr) {
            Ok(val) => println!("  {expr:<32} = {val}"),
            Err(e) => println!("  {expr:<32} => ERROR: {e}"),
        }
    }

    Ok(())
}
