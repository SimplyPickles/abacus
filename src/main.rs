use abacus::{Abacus, AbacusError, BinaryOp, Value};

fn main() -> Result<(), AbacusError> {
    let mut calc = Abacus::standard();

    println!("============================================================");
    println!("             ABACUS CALCULATOR ENGINE SHOWCASE              ");
    println!("============================================================\n");

    // 1. Basic & Derived Physical Unit Arithmetic
    println!("--- 1. Physical Unit Arithmetic ---");
    println!(
        "  10 N * 5 m                   = {}",
        calc.eval("10 N * 5 m")?.to_display()
    );
    println!(
        "  100 km / 2 h to m/s          = {}",
        calc.eval("100 km / 2 h to m/s")?.to_display()
    );
    println!(
        "  500 J / 10 s                 = {}\n",
        calc.eval("500 J / 10 s")?.to_display()
    );

    // 2. Guaranteed Physical Interval Arithmetic
    println!("--- 2. Guaranteed Physical Interval Arithmetic ---");
    println!(
        "  [9.8 m, 10.2 m] / [1.9 s, 2.1 s] = {}",
        calc.eval("[9.8 m, 10.2 m] / [1.9 s, 2.1 s]")?.to_display()
    );
    println!(
        "  5 m + [1 m, 3 m]             = {}",
        calc.eval("5 m + [1 m, 3 m]")?.to_display()
    );
    println!(
        "  [1 km, 2 km] to m            = {}",
        calc.eval("[1 km, 2 km] to m")?.to_display()
    );
    println!(
        "  [10 N, 20 N] * [2 m, 5 m]    = {}\n",
        calc.eval("[10 N, 20 N] * [2 m, 5 m]")?.to_display()
    );

    // 3. TI-84 Inferential Statistics & Confidence Intervals
    println!("--- 3. TI-84 Inferential Statistics & Confidence Intervals ---");
    println!(
        "  TInterval(10 m, 12 m, 11 m, 14 m) = {}",
        calc.eval("TInterval(10 m, 12 m, 11 m, 14 m)")?.to_display()
    );
    println!(
        "  ZInterval(100 m, 15 m, 100)       = {}",
        calc.eval("ZInterval(100 m, 15 m, 100)")?.to_display()
    );
    println!(
        "  1-PropZInt(45, 100)               = {}",
        calc.eval("1-PropZInt(45, 100)")?.to_display()
    );
    println!(
        "  2-SampTInt(100 m, 15 m, 25, 90 m, 10 m, 30) = {}\n",
        calc.eval("2-SampTInt(100 m, 15 m, 25, 90 m, 10 m, 30)")?
            .to_display()
    );

    // 4. TI-84 Hypothesis Testing
    println!("--- 4. TI-84 Hypothesis Testing ---");
    println!(
        "  ZTest(100 m, 105 m, 15 m, 50)           = {}",
        calc.eval("ZTest(100 m, 105 m, 15 m, 50)")?.to_display()
    );
    println!(
        "  ZTest(100 m, 105 m, 15 m, 50).p_value   = {}",
        calc.eval("ZTest(100 m, 105 m, 15 m, 50).p_value")?.to_display()
    );
    println!(
        "  TTest(100 m, 105 m, 15 m, 25)           = {}",
        calc.eval("TTest(100 m, 105 m, 15 m, 25)")?.to_display()
    );
    println!(
        "  2-SampTTest(100 m, 15 m, 25, 90 m, 10 m, 30) = {}",
        calc.eval("2-SampTTest(100 m, 15 m, 25, 90 m, 10 m, 30)")?.to_display()
    );
    println!(
        "  Chi2Test(15, 25, 10, 30).p_value        = {}\n",
        calc.eval("Chi2Test(15, 25, 10, 30).p_value")?.to_display()
    );

    // 5. Dimension-Aware Linear Regression & Natural Language Dot Property Access
    println!("--- 5. Dimension-Aware Linear Regression & Dot Property Access ---");
    println!(
        "  linreg(1 s, 2 s, 3 s, 4 s, 10 m, 20 m, 30 m, 40 m)           = {}",
        calc.eval("linreg(1 s, 2 s, 3 s, 4 s, 10 m, 20 m, 30 m, 40 m)")?
            .to_display()
    );
    println!(
        "  linreg(1 s, 2 s, 3 s, 4 s, 15 m, 25 m, 35 m, 45 m).intercept = {}",
        calc.eval("linreg(1 s, 2 s, 3 s, 4 s, 15 m, 25 m, 35 m, 45 m).intercept")?
            .to_display()
    );
    println!(
        "  linreg(1 s, 2 s, 3 s, 4 s, 10 m, 20 m, 30 m, 40 m).slope     = {}",
        calc.eval("linreg(1 s, 2 s, 3 s, 4 s, 10 m, 20 m, 30 m, 40 m).slope")?
            .to_display()
    );
    println!(
        "  linreg(1 s, 2 s, 3 s, 4 s, 10 m, 20 m, 30 m, 40 m).slope * 5 s = {}\n",
        calc.eval("linreg(1 s, 2 s, 3 s, 4 s, 10 m, 20 m, 30 m, 40 m).slope * 5 s")?
            .to_display()
    );

    // 6. Custom Operator Registration
    println!("--- 6. Custom Operator Registration ---");
    fn custom_add_ten(lhs: Value, rhs: Value) -> Result<Value, AbacusError> {
        let sum = (&lhs + &rhs)?;
        Ok(Value::new(sum.canonical + 10.0, sum.unit))
    }

    calc.register_binop_token(BinaryOp {
        alias: "~",
        func: custom_add_ten,
        precedence: 10,
        right_associative: false,
    });

    println!(
        "  5 ~ 3                        = {}",
        calc.eval("5 ~ 3")?.to_display()
    );
    println!("\n============================================================");

    Ok(())
}
