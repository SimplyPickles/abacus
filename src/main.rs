use abacus::{AbacusError, eval};

fn main() -> Result<(), AbacusError> {
    println!("{}", eval("5m+5m")?);
    println!("{}", eval("5 m + 5")?);
    println!("{}", eval("5 cm + 5")?);
    println!("{}", eval("(10+5)(2+3)")?);

    Ok(())
}
