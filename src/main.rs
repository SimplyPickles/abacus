use abacus::{
    evaluation::parser::parse::evaluate,
    evaluation::tokenizer::registry::token_registry::TokenRegistry,
    AbacusError, UnitRegistry,
};

fn main() -> Result<(), AbacusError> {
    let token_reg = TokenRegistry::standard();
    let unit_reg = UnitRegistry::standard();

    let expressions = [
        "// Basic Arithmetic & Units",
        "5 m + 3 m",
        "(5 m + 20 cm) as m",
        "2 + 3 * 4",
        "sqrt(9 m^2)",
        "sqrt(5 m)",
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
        "// Continuous Distributions",
        "normpdf(0)",
        "normcdf(0)",
        "normcdf(70 kg, 65 kg, 5 kg)",
        "tcdf(10, 2.228)",
        "chisqcdf(10, 18.307)",
        "fcdf(5, 10, 3.33)",
        "expcdf(0.5, 2)",
        "unifcdf(0, 10, 5)",
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
