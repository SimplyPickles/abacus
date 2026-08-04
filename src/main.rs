use abacus::{AbacusError, eval};

fn main() -> Result<(), AbacusError> {
    println!("{}", eval("5m+5m")?);
    println!("{}", eval("5 m + 5")?);
    println!("{}", eval("5 cm + 5")?);
    println!("{}", eval("(10+5)(2+3)")?);
    println!("10 N * 5 m = {}", eval("10 N * 5 m")?);
    println!("100 W * 5 s = {}", eval("100 W * 5 s")?);
    println!("12 V * 2 A = {}", eval("12 V * 2 A")?);
    println!("10 N / 2 m^2 = {}", eval("10 N / 2 m^2")?);
    println!("5 C / 2 s = {}", eval("5 C / 2 s")?);

    Ok(())
}
