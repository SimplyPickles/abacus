use abacus::{AbacusError, UnitRegistry};

fn main() -> Result<(), AbacusError> {
    let registry = UnitRegistry::standard();

    let speed = registry.value(5.0, "m")? / registry.value(1.0, "s")?;

    let distance = (speed? * registry.value(5.0, "s")?)?;

    let mass = registry.value(2.0, "kg")?;
    let accel =
        registry.value(9.8, "m")? / (registry.value(1.0, "s")? * registry.value(1.0, "s")?)?;
    let force = (mass * accel?)?;
    // Automatic formula-to-derived-unit conversion: kg*m/s^2 -> N
    let force_derived = force.to_derived(&registry)?;

    // Volume addition and conversion using &Value reference arithmetic
    let barrels = registry.value(1.0, "bbl")?;
    let liters = registry.value(100.0, "L")?;
    let total_volume = (&barrels + &liters)?;
    let volume_m3 = total_volume.to(&registry, "m^3")?;

    println!("Distance: {distance}");
    println!("Force (auto-derived): {force_derived}");
    println!("1 bbl ({barrels}) + 100 L ({liters}) in m^3: {volume_m3}");

    Ok(())
}
