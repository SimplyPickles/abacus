use abacus::{AbacusError, eval};

fn main() -> Result<(), AbacusError> {
    println!("6min +0.5hours- 1s: {}", eval("6min +0.5h- 1s")?);

    Ok(())
}
