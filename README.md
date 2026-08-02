# Abacus

Abacus is an experimental zero-dependency Rust units-and-values calculator. It stores values in canonical units, tracks physical dimensions, supports arithmetic across compatible units, and offers extensive registries covering SI, derived, storage, Imperial, US customary, astronomical, nautical, CGS, and niche units.

## Features

- **Canonical Unit Storage**: Values are stored as floats in SI base units.
- **Dimension-Aware Arithmetic**: Tracks physical dimensions across addition, subtraction, multiplication, and division.
- **`UnitRegistry` API**: Encapsulated registry for unit lookup, value construction, and dynamic unit resolution.
- **Dynamic Exponent Units**: Supports automatic resolution of exponent units like `m^3`, `cm^3`, `ft^3`, `m^2`, `ft^2`, etc.
- **Ergonomic Conversions**: Convert values directly using `.to(&registry, "symbol")`.
- **Affine Unit Protection**: Guarded conversion and arithmetic for affine units like Celsius and Fahrenheit.
- **Strongly Typed Errors**: Domain-specific `AbacusError` enum.
- **Extensive Registries**:
  - Metric/SI base and derived units (`N`, `J`, `W`, `Pa`, `Hz`, `V`, `Ω`, `F`, etc.)
  - Metric volume (`L`, `mL`, `kL`) and land area (`ha`, `a`)
  - Storage units with decimal (`kB`, `MB`) and IEC binary (`KiB`, `MiB`) prefixes
  - Fixed-duration time units (`min`, `h`, `d`, `wk`)
  - British Imperial and US Customary liquid measures (`us_gal`, `bbl`, `fl oz`)
  - Angle units (`rad`, `deg`, `turn`, `arcmin`, `arcsec`)
  - Astronomical, Nautical, CGS physics, Typography, Computing niche, Historical, and Humorous units

---

## Core Concepts

### `UnitRegistry`

The `UnitRegistry` manages registered unit definitions and dynamic unit exponent lookups:

```rust
let registry = UnitRegistry::standard();
let meter = registry.unit("m")?;
let distance = registry.value(5.0, "km")?;
```

### `Unit`

A `Unit` describes how a displayed unit maps to canonical representation:

```rust
pub struct Unit {
    pub dimensions: Dimensions,
    pub scalar: f64,
    pub offset: f64,
    pub display: UnitExpr,
}
```

- `dimensions`: Physical dimension array `[length, mass, time, current, temp, amount, luminous, info]`.
- `scalar`: Converts from displayed units to canonical SI units.
- `offset`: Supports affine units like Celsius and Fahrenheit.
- `display`: Structured unit expression for rendering compound units.

### `Value`

A `Value` stores a canonical numeric value plus the shared `Arc<Unit>` display representation:

```rust
let distance = registry.value(5.0, "km")?;
let meters = distance.to(&registry, "m")?;

assert_eq!(meters.to_display(), "5000 m");
```

---

## Example

In `src/main.rs`:

```rust
fn main() -> Result<(), AbacusError> {
    let registry = UnitRegistry::standard();

    // Speed and distance calculation
    let speed = registry.value(5.0, "m")? / registry.value(1.0, "s")?;
    let distance = (speed? * registry.value(5.0, "s")?)?;

    // Force calculation using derived SI units
    let mass = registry.value(2.0, "kg")?;
    let accel = registry.value(9.8, "m")? / (registry.value(1.0, "s")? * registry.value(1.0, "s")?)?;
    let force = (mass * accel?)?;
    let force_newtons = force.to(&registry, "N")?;

    // Volume addition and conversion to m^3
    let barrels = registry.value(1.0, "bbl")?;
    let liters = registry.value(100.0, "L")?;
    let total_volume = (barrels + liters)?;
    let volume_m3 = total_volume.to(&registry, "m^3")?;

    println!("Distance: {}", distance.to_display());
    println!("Force: {}", force_newtons.to_display());
    println!("1 bbl + 100 L in m^3: {}", volume_m3.to_display());

    Ok(())
}
```

Output:

```text
Distance: 25 m
Force: 19.6 N
1 bbl + 100 L in m^3: 0.258987294928 m^3
```

---

## Arithmetic & Conversions

### Addition and Subtraction

Addition and subtraction require compatible dimensions:

```rust
let a = registry.value(1.0, "km")?;
let b = registry.value(500.0, "m")?;

let result = (a + b)?;
assert_eq!(result.to_display(), "1.5 km");
```

### Volume Additions and Conversions

```rust
let bbl = registry.value(1.0, "bbl")?;
let liters = registry.value(100.0, "L")?;

let total = (bbl + liters)?.to(&registry, "m^3")?;
assert_eq!(total.to_display(), "0.258987294928 m^3");
```

### Temperature & Affine Units

Affine units such as Celsius and Fahrenheit require both a scalar and an offset. Abacus supports converting affine units into compatible units, but guards against illegal arithmetic:

```rust
let celsius = registry.value(100.0, "°C")?;
let kelvin = celsius.to(&registry, "K")?;
let fahrenheit = celsius.to(&registry, "°F")?;

assert_eq!(kelvin.to_display(), "373.15 K");
assert_eq!(fahrenheit.to_display(), "212 °F");
```

---

## Unit Families Included

- **SI Base Units**: `s`, `m`, `g`, `A`, `K`, `mol`, `cd` (plus all standard metric prefixes)
- **SI Derived Units**: `Hz`, `N`, `Pa`, `J`, `W`, `C`, `V`, `F`, `Ω`, `S`, `Wb`, `T`, `H`, `lm`, `lx`, `Bq`, `Gy`, `Sv`, `kat`
- **Metric Volume & Area**: `L`, `mL`, `cL`, `dL`, `kL`, `ha` (hectare), `a` (are)
- **Storage**: `b`, `B`, `kB`, `MB`, `GB`, `KiB`, `MiB`, `GiB`
- **British Imperial**: `in`, `ft`, `yd`, `mi`, `ac`, `lb`, `oz`, `gal`, `qt`, `pt`, `fl oz`
- **US Customary**: `us_gal`, `us_qt`, `us_pt`, `us_fl_oz`, `cup`, `tbsp`, `tsp`
- **Angles**: `rad`, `deg`, `°`, `turn`, `arcmin`, `arcsec`
- **Astronomical**: `au`, `ly`, `pc` (`kpc`, `Mpc`), `solar_mass`, `jansky`
- **Nautical**: `nmi`, `knot`, `fathom`, `cable`, `rod`, `link`, `league`
- **CGS Physics**: `Å` (angstrom), `eV` (`keV`, `MeV`, `GeV`, `TeV`), `dalton`, `bar`, `atm`, `torr`, `barn`, `gauss`, `maxwell`, `poise`, `stokes`, `galileo`
- **Typography**: `point`, `pica`, `twip`
- **Computing Niche**: `nibble`, `crumb`, `word`, `dword`, `qword`, `shannon`, `hartley`, `nat`
- **Trade & Historical**: `bbl` (oil barrel), `hogshead`, `carat`, `troy_ounce`, `slug`, `poundal`
- **Humorous**: `smoot`, `shake`, `jiffy`, `fortnight`, `furlong_per_fortnight`, `barn_megaparsec`, `attoparsec`

---

## Running & Testing

```sh
cargo run
```

```sh
cargo test
```

All 48 unit tests cover unit registration, value conversions, derived unit arithmetic, volume additions, affine unit guards, and exponent resolution.

---

## Current Limitations

- **Expression Parser**: No text expression parser yet (e.g. parsing `"5km / 1h"` directly from a raw string input). Values are currently constructed programmatically through `registry.value(...)`.
