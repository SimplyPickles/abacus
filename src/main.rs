use abacus::{Abacus, AbacusError, BinaryOp, Value};

fn main() -> Result<(), AbacusError> {
    let mut calc = Abacus::standard();

    fn add_ten(lhs: Value, rhs: Value) -> Result<Value, abacus::AbacusError> {
        let sum = (&lhs + &rhs)?;
        Ok(Value::new(sum.canonical + 10.0, sum.unit))
    }

    calc.register_binop_token(BinaryOp {
        alias: "~",
        func: add_ten,
        precedence: 10,
        right_associative: false,
    });

    println!("{}", calc.eval("5 ~ 3").unwrap().to_display());

    Ok(())
}
