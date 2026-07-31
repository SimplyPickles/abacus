# Abacus

Abacus is an experimental Rust units-and-values calculator. It stores values in canonical units, tracks physical dimensions, and supports arithmetic across compatible units.

## Features

- Canonical unit storage
- Dimension-aware arithmetic
- Unit conversion through scalar factors
- Affine temperature unit support
- Structured compound unit display
- Metric/SI base units and prefixes
- Fixed-duration time units
- Storage units with decimal and binary prefixes
- British Imperial units

## Core concepts

### `Unit`

A `Unit` describes how a displayed unit maps to the canonical representation:

```rust
pub struct Unit {
    pub dimensions: Dimensions,
    pub scalar: f64,
    pub offset: f64,
    pub display: UnitExpr,
}
```

- `dimensions` tracks the physical dimension, such as length, time, mass, or information.
- `scalar` converts from displayed units to canonical units.
- `offset` supports affine units like Celsius and Fahrenheit.
- `display` stores a structured unit expression for rendering compound units.

For example:

```text
1 km = 1000 canonical meters
1 hour = 3600 canonical seconds
1 B = 8 canonical bits
```

### `Value`

A `Value` stores a canonical numeric value plus the unit it should display in:

```rust
pub struct Value {
    pub canonical: f64,
    pub unit: Arc<Unit>,
}
```

Use `Value::new(...)` to construct values from displayed quantities:

```rust
let distance = Value::new(5.0, Arc::clone(metric_units().get("km").unwrap()));
let duration = Value::new(1.0, Arc::clone(metric_units().get("h").unwrap()));
```

This stores:

```text
5 km => 5000 canonical meters
1 h  => 3600 canonical seconds
```

## Example

`src/main.rs` currently demonstrates:

```rust
let v1 = Value::new(5.0, Arc::clone(metric_units().get("km").unwrap()));
let v2 = Value::new(1.0, Arc::clone(metric_units().get("hour").unwrap()));
let speed = (v1 / v2).unwrap();

println!("{}", speed.to_display());
```

Output:

```text
5km/h
```

## Arithmetic

Arithmetic uses canonical values and combines unit dimensions.

### Addition and subtraction

Addition and subtraction require compatible dimensions:

```rust
let a = Value::new(1.0, Arc::clone(metric_units().get("km").unwrap()));
let b = Value::new(500.0, Arc::clone(metric_units().get("m").unwrap()));

let result = (a + b).unwrap();
assert_eq!(result.to_display(), "1.5km");
```

### Multiplication and division

Multiplication adds dimensions; division subtracts dimensions:

```text
m / s => length per time
m * m => area
m * m * m => volume
```

Compound unit display is simplified symbolically:

```text
m/s * s => m
m * m   => m^2
m^3 / s => m^3/s
```

## Unit display expressions

`UnitExpr` represents compound unit displays as numerator and denominator symbols:

```rust
pub struct UnitExpr {
    pub numerator: Vec<String>,
    pub denominator: Vec<String>,
}
```

It supports:

- `multiply(...)`
- `divide(...)`
- `simplified()`
- `render()`

Examples:

```text
UnitExpr::single("m").divide(&UnitExpr::single("s")) => "m/s"
UnitExpr::single("m").multiply(&UnitExpr::single("m")) => "m^2"
```

Simplification currently cancels exact symbol matches only. For example:

```text
m/s * s => m
```

But equivalent units with different display symbols do not cancel display-wise:

```text
m/min * s
```

The canonical value and dimensions are still correct.

## Registered units

### Metric/SI base units

- `s` second
- `m` meter
- `g` gram
- `A` ampere
- `K` kelvin
- `mol` mole
- `cd` candela

Metric prefixes are generated for SI base units, including:

```text
km, cm, mm, μs, kg, MB-style prefixes for applicable units, etc.
```

### Time units

- `min`, `minute`
- `h`, `hour`
- `d`, `day`
- `wk`, `week`

### Storage units

Canonical storage is bits.

Base units:

- `b`, `bit`
- `B`, `byte`

Decimal units:

- `kb`, `kB`
- `Mb`, `MB`
- `Gb`, `GB`
- through `Qb`, `QB`

Binary IEC units:

- `Kib`, `KiB`
- `Mib`, `MiB`
- `Gib`, `GiB`
- through `Yib`, `YiB`

### British Imperial units

Length:

- `in`, `inch`, `inches`
- `ft`, `foot`, `feet`
- `yd`, `yard`, `yards`
- `ch`, `chain`, `chains`
- `fur`, `furlong`, `furlongs`
- `mi`, `mile`, `miles`

Area:

- `ac`, `acre`, `acres`

Mass:

- `gr`, `grain`
- `dr`, `dram`
- `oz`, `ounce`
- `lb`, `pound`
- `st`, `stone`
- `cwt`, `hundredweight`
- `ton`, `long ton`, `imperial ton`

British Imperial volume:

- `fl oz`, `floz`, `fluid ounce`
- `gi`, `gill`
- `pt`, `pint`
- `qt`, `quart`
- `gal`, `gallon`

Temperature:

- `K`, `kelvin`
- `°C`, `degC`, `celsius`
- `°F`, `degF`, `fahrenheit`

> Note: British Imperial liquid units differ from US customary liquid units. `gal` refers to the British Imperial gallon.

## Temperature and affine units

Affine units such as Celsius and Fahrenheit require both a scalar and an offset.

Abacus supports converting affine units into compatible units, but rejects arithmetic that would produce misleading results:

```rust
let celsius = Value::new(100.0, Arc::clone(metric_units().get("°C").unwrap()));
let kelvin = celsius
    .convert_to(Arc::clone(metric_units().get("K").unwrap()))
    .unwrap();
let fahrenheit = celsius
    .convert_to(Arc::clone(metric_units().get("°F").unwrap()))
    .unwrap();

assert_eq!(kelvin.to_display(), "373.15K");
assert_eq!(fahrenheit.to_display(), "212°F");
```

Unsupported operations such as directly multiplying or subtracting affine temperature values return errors.

## Running

```sh
cargo run
```

Current output:

```text
5km / 1h
5km/h
```

## Testing

```sh
cargo test
```

The test suite covers:

- unit registration
- time unit scalars
- storage units
- value arithmetic
- compound unit simplification
- exponent display such as `m^2` and `m^3`

## Current limitations

- No expression parser yet. Values are currently constructed manually in Rust.
- Unit display simplification only cancels exact symbols.
- No normalized display selection yet, such as automatically converting `m*cm` to `m^2`.
- Some common derived SI units are not registered yet.
- US customary volume units are not registered yet.
