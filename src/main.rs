use abacus::{Abacus, AbacusError};

fn main() -> Result<(), AbacusError> {
    let calc = Abacus::standard();
    println!("{}", calc.eval("50km + 10km")?);

    Ok(())
}
