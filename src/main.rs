use abacus::{Abacus, AbacusError, BinaryOp, Value};

fn main() -> Result<(), AbacusError> {
    let calc = Abacus::standard();

    // The flagship example
    println!("[9.8 m, 10.2 m] / [1.9 s, 2.1 s] = {}", calc.eval("[9.8 m, 10.2 m] / [1.9 s, 2.1 s]").unwrap().to_display()); // → [4.666... m/s, 5.368... m/s]

    // Mixed scalar + interval
    println!("[5 m + [1 m, 3 m]] = {}", calc.eval("5 m + [1 m, 3 m]").unwrap().to_display()); // → [6 m, 8 m]

    // Unit conversion
    println!("[1 km, 2 km] to m = {}", calc.eval("[1 km, 2 km] to m").unwrap().to_display()); // → [1000 m, 2000 m]

    // Derived unit reduction
    println!("[10 N, 20 N] * [2 m, 5 m] = {}", calc.eval("[10 N, 20 N] * [2 m, 5 m]").unwrap().to_display()); // → [20 J, 100 J]

    // Zero-crossing intervals
    println!("[[-2, 3] * [1, 4]] = {}", calc.eval("[-2, 3] * [1, 4]").unwrap().to_display()); // → [-8, 12]

    // Implicit multiplication
    println!("[5 [1, 3]] = {}", calc.eval("5 [1, 3]").unwrap().to_display()); // → [5, 15]

    Ok(())
}
