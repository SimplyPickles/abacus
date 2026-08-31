# Abacus

Abacus is a zero-dependency, unit-aware mathematical evaluation engine written in Rust. It supports physical dimensions, implicit multiplication, unit conversion, intervals, ranges, statistics, probability distributions, confidence intervals, financial functions, and dimension-aware linear regression.

## Features

* **Pratt parser** — operator precedence, grouping, unary/binary/postfix operators, implicit multiplication, dot property access, and `as` / `in` / `to` conversions.
* **Units & dimensions** — SI, imperial, US customary, astronomical, CGS, storage, angle, typography, historical, and other units. Dimensions use an 8-component basis: `[length, mass, time, current, temperature, amount, luminous, information]`.
* **Implicit multiplication** — `5(2 + 3)`, `(2 + 3)(4 + 5)`, `2 sqrt(9 m^2)`.
* **Interval arithmetic** — `[lo, hi]` intervals with physical units and worst/best-case bounds.
* **Derived SI reduction** — `10 N * 5 m -> 50 J`, `100 W * 5 s -> 500 J`, `12 V * 2 A -> 24 W`.
* **Unit promotion & cancellation** — `5 m + 5 -> 10 m`, `1 as inches -> 1 in`, `5 km / m -> 5000`.
* **Fractional dimensions** — powers and square roots preserve dimensional information.
* **Statistics** — descriptive statistics, variance, dispersion, covariance, correlation, skewness, kurtosis, z-scores, quantiles, and percentiles.
* **Probability & distributions** — probability distributions and inverse CDFs.
* **Confidence intervals** — TI-84-style `TInterval`, `ZInterval`, `1-PropZInt`, `2-SampTInt`, `2-SampZInt`, and `2-PropZInt`.
* **Linear regression** — dimension-aware `linreg` with slope, intercept, correlation, `R²`, standard error, means, and predictions.
* **Financial functions** — `pmt`, `fv`, `pv`, `npv`, `irr`, and `compound`.
* **Math library** — trigonometric, hyperbolic, logarithmic, exponential, rounding, and general math functions.
* **Custom functions** — register scalar functions or functions returning `Scalar`, `Interval`, or `Hash` results.

## Quickstart

Add Abacus to `Cargo.toml`:

```toml
[dependencies]
abacus = "..."
```

```rust
use abacus::Abacus;

fn main() {
    let calc = Abacus::standard();

    println!("{}", calc.eval("10 N * 5 m").unwrap());
    // 50 J

    println!("{}", calc.eval("100 km / 2 h to m/s").unwrap());
    // 13.88888888888889 m/s

    println!(
        "{}",
        calc.eval("[9.8 m, 10.2 m] / [1.9 s, 2.1 s]").unwrap()
    );
    // [4.666 m/s, 5.368 m/s]

    println!(
        "{}",
        calc.eval(
            "linreg(1 s, 2 s, 3 s, 4 s, 10 m, 20 m, 30 m, 40 m).slope * 5 s"
        ).unwrap()
    );
    // 50 m
}
```

## Expression Examples

| Expression                         | Result                   |
| ---------------------------------- | ------------------------ |
| `5 m + 3 m`                        | `8 m`                    |
| `(5 m + 20 cm) as m`               | `5.2 m`                  |
| `10 N * 5 m`                       | `50 J`                   |
| `5 km to m`                        | `5000 m`                 |
| `1 m in inches`                    | `39.37007874 in`         |
| `[9.8 m, 10.2 m] / [1.9 s, 2.1 s]` | `[4.666 m/s, 5.368 m/s]` |
| `1m / m`                           | `1`                      |
| `5 km / m`                         | `5000`                   |
| `floor(5.7 m)`                     | `5 m`                    |

## Ranges

Functions accept individual values, ranges, and stepped ranges:

```text
sum(1..5)
mean(1..5)
std(1..9..2)

0 m .. 10 m .. 2 m
0 km .. 1 km .. 250 m
```

## Statistics

### Descriptive

```text
sum  mean  geomean  harmean  median  mode
min  max   range    mad      rms
```

### Variance and dispersion

```text
var  var_s  var_p
std  std_s  std_p
quantile  percentile  iqr
```

### Bivariate and shape

```text
cov  cov_s  cov_p
corr
skew  skewness
kurt  kurtosis
zscore  standardize
```

## Linear Regression

`linreg` preserves the physical dimensions of both variables:

```text
linreg(
    1 s, 2 s, 3 s, 4 s,
    10 m, 20 m, 30 m, 40 m
)
```

```text
{
    intercept: 0 m,
    mean_x: 2.5 s,
    mean_y: 25 m,
    r: 1,
    r2: 1,
    se: 0 m,
    slope: 10 m/s
}
```

Hash properties can be accessed with dot notation:

```text
linreg(1 s, 2 s, 3 s, 4 s, 10 m, 20 m, 30 m, 40 m).slope
```

```text
10 m/s
```

## Confidence Intervals

```text
TInterval(10 m, 12 m, 11 m, 14 m)
-> [9.032 m, 14.468 m]

ZInterval(100 m, 15 m, 100)
-> [97.061 m, 102.939 m]

1-PropZInt(45, 100)
-> [0.356, 0.548]

2-SampTInt(100 m, 15 m, 25, 90 m, 10 m, 30)
-> [2.905 m, 17.095 m]
```

## Mathematics

| Category      | Functions                                            |
| ------------- | ---------------------------------------------------- |
| Trigonometric | `sin`, `cos`, `tan`, `asin`, `acos`, `atan`, `atan2` |
| Hyperbolic    | `sinh`, `cosh`, `tanh`, `asinh`, `acosh`, `atanh`    |
| Logarithmic   | `ln`, `log10`, `log2`, `log(x, base)`, `exp`         |
| Rounding      | `abs`, `floor`, `ceil`, `round`, `sign`              |
| Combinatorics | `!`, `factorial`, `nCr`, `comb`, `nPr`, `perm`       |
| Financial     | `pmt`, `fv`, `pv`, `npv`, `irr`, `compound`          |

Supported angle units include `rad`, `deg`, `°`, `turn`, `arcmin`, and `arcsec`.

## Units

The registry includes SI base and derived units, metric prefixes, imperial/US customary units, storage units, angles, astronomical and nautical units, CGS physics units, typography units, and historical units.

Examples:

```text
m  kg  s  N  J  W  Pa  Hz  V  Ω  F  Wb  T  H
in  ft  yd  mi  lb  oz  us_gal  bbl  fl oz
kB  MB  KiB  MiB
au  ly  pc  solar_mass
eV  Å  bar  gauss
smoot  fortnight  attoparsec
```

Exponent units such as `m^2`, `m^3`, `cm^3`, and `ft^3` are resolved dynamically. Affine units such as `°C` and `°F` receive special handling.

## Custom Functions

Scalar functions can be registered with `FunctionTarget::Scalar`:

```rust
use abacus::{Abacus, AbacusError, FunctionOp, FunctionTarget, Value};

fn double_val(args: &[Value]) -> Result<Value, AbacusError> {
    if args.len() != 1 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }

    let value = &args[0];
    Ok(Value::new(value.canonical * 2.0, value.unit.clone()))
}

fn main() {
    let mut calc = Abacus::standard();

    calc.tokens.function_operators.insert(
        "double",
        FunctionOp {
            name: "double",
            min_args: 1,
            max_args: 1,
            func: FunctionTarget::Scalar(double_val),
        },
    );

    println!("{}", calc.eval("double(5 m)").unwrap());
    // 10 m
}
```

Functions returning structured results can use `FunctionTarget::EvalResult` and return `EvalResult::Hash`. Hash fields are accessible through dot notation:

```text
summary(5 m).doubled
summary(5 m).squared
```

## Testing

Run the test suite:

```sh
cargo test
```

The tests cover parser precedence, implicit multiplication, unit arithmetic and conversion, interval arithmetic, unit cancellation, fractional dimensions, ranges, statistics, probability distributions, inverse CDFs, confidence intervals, regression, and property access.

Run the REPL:

```sh
cargo run
# or
cargo run --bin repl
```

Run the example binary:

```sh
cargo run --bin abacus
```
