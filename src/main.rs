use abacus::{AbacusError, eval};

fn main() -> Result<(), AbacusError> {
    println!("50 J to N*m = {}", eval("50 J to N*m")?);

    Ok(())
}
