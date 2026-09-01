# Abacus

Abacus is a blazing fast, zero-dependency (core), unit-aware mathematical evaluation engine written in Rust. It provides a Pratt parser supporting physical dimensions, implicit multiplication, unit conversions, intervals, date & relative time calculations, business day calendars, statistics, probability distributions, hypothesis testing, confidence intervals, financial mathematics, and dimension-aware linear regression.

---

## Features

- **Pratt Parser** — Operator precedence, grouping, unary/binary/postfix operators, implicit multiplication (`5(2 + 3)`, `2 sqrt(9 m^2)`), dot property access, and `as` / `in` / `to` conversions.
- **Units & Dimensions** — SI base & derived, imperial, US customary, astronomical, nautical, CGS physics, storage, typography, and historical units. Dimensions use a compact 16-byte fixed-point representation (`[i16; 8]`) fitting in a single 128-bit SIMD register.
- **Interval Arithmetic** — Both bracket (`[1, 10]`) and range (`1..10`) interval syntax with physical units, monotonic fast-paths for addition/subtraction, and singularity handling for division crossing zero.
- **Dates & Natural Relative Time** — Parse and calculate with dates (`07-08-2026`, `2026-08-07`), times (`15:30:00`, `3:30 PM`), timezones (`UTC`, `EST`, `PST`, `+02:00`), and natural language expressions (`"last thursday at 3pm + 2 weeks"`, `"today at 12 to 1"`).
- **Business Day Calendars** — $O(1)$ closed-form business day arithmetic (`07-08-2026 + 5 business days`, `10-08-2026 - 1 business day`), interval counting (`workdays(start, end)`), and workday/weekend predicates.
- **Derived SI Reduction** — Automatic reduction to standard derived units (`10 N * 5 m -> 50 J`, `100 W * 5 s -> 500 J`, `12 V * 2 A -> 24 W`).
- **Statistics & Distributions** — Descriptive statistics, dispersion, covariance, correlation, skewness, kurtosis, z-scores, quantiles, and probability distributions (Normal, Student's t, Binomial, Poisson with log-space overflow protection, Chi-Squared, F).
- **Hypothesis Testing & Confidence Intervals** — TI-84-style tests (`ZTest`, `TTest`, `2-SampZTest`, `2-SampTTest`, `1-PropZTest`, `2-PropZTest`, `Chi2Test`) and intervals (`ZInterval`, `TInterval`, `1-PropZInt`, `2-SampTInt`, `2-SampZInt`, `2-PropZInt`).
- **Linear Regression** — Dimension-aware `linreg` returning a structured hash containing `slope`, `intercept`, `r`, `r2`, `se`, means, and prediction calculations.
- **Financial Functions** — `pmt`, `fv`, `pv`, `npv`, `irr`, and `compound`.
- **Zero-Allocation Architecture** — Global lazy registry caching (`abacus::eval`), shared static dimensionless unit pool, zero-allocation lexing, and direct `Display` formatter streaming.
- **Modular Cargo Features & Serde** — Fine-grained feature flags (`units`, `stats`, `distributions`, `date`, `financial`, `serde`, `repl`).

---

## Quickstart

Add Abacus to your `Cargo.toml`:

```toml
[dependencies]
abacus = "0.1"
```

### Basic Usage

```rust
use abacus::eval;

fn main() {
    // One-off zero-allocation evaluation via global cached engine:
    let res = eval("10 N * 5 m").unwrap();
    println!("{res}"); // 50 J

    let res = eval("100 km/h in m/s").unwrap();
    println!("{res}"); // 27.77777777777778 m/s

    // Interval arithmetic
    let res = eval("1..10 + 5").unwrap();
    println!("{res}"); // 6..15

    let res = eval("[9.8 m, 10.2 m] / [1.9 s, 2.1 s]").unwrap();
    println!("{res}"); // [4.666666666666667 m/s, 5.368421052631579 m/s]

    // Dates and natural relative time
    let res = eval("07-08-2026 + 5 business days").unwrap();
    println!("{res}"); // 14-08-2026

    // Dimension-aware linear regression
    let res = eval("linreg(1 s, 2 s, 3 s, 4 s, 10 m, 20 m, 30 m, 40 m).slope * 5 s").unwrap();
    println!("{res}"); // 50 m
}
```

### Engine Configuration

Configure significant figures and unit behaviors directly on the `Abacus` instance:

```rust
use abacus::Abacus;

let mut calc = Abacus::standard();

// 1. Configurable Significant Figures (rounding & formatting)
let calc_sig = Abacus::standard().with_significant_figures(3);
println!("{}", calc_sig.eval("12.3456 m").unwrap()); // 12.3 m
println!("{}", calc_sig.eval("12345.6 m").unwrap()); // 12300 m

// 2. Automatically follow input significant figures
let calc_follow = Abacus::standard().with_follow_significant_figures(true);
println!("{}", calc_follow.eval("12.3 * 4.567").unwrap()); // 56.2 (3 sig figs)

// 3. Toggle Automatic Derived Unit Reduction
let calc_raw = Abacus::standard().with_auto_derived_units(false);
println!("{}", calc_raw.eval("10 N * 5 m").unwrap()); // 50 N*m (not reduced to J)
```

---

## Examples

### Physical Units & Conversions

| Expression           | Result                 | Description                              |
| :------------------- | :--------------------- | :--------------------------------------- |
| `5 m + 3 m`          | `8 m`                  | Basic addition with identical units      |
| `(5 m + 20 cm) as m` | `5.2 m`                | Mixed prefix conversion                  |
| `10 N * 5 m`         | `50 J`                 | Automatic reduction to derived SI unit   |
| `100 W * 5 s`        | `500 J`                | Power $\times$ time energy reduction     |
| `12 V * 2 A`         | `24 W`                 | Voltage $\times$ current power reduction |
| `5 km to m`          | `5000 m`               | Explicit unit conversion                 |
| `1 m in inches`      | `39.37007874015748 in` | Metric to Imperial conversion            |
| `5 km / m`           | `5000`                 | Dimensionless ratio cancellation         |
| `100 °C to °F`       | `212 °F`               | Affine temperature scale conversion      |

### Interval Arithmetic

Intervals support both bracket syntax `[a, b]` and range syntax `a..b`:

```text
[1 m, 2 m] + [3 m, 4 m]  -> [4 m, 6 m]
1..10 + 5                -> 6..15
[10 m, 20 m] - [2 m, 5 m]-> [5 m, 18 m]
[9.8 m, 10.2 m] / 2 s    -> [4.9 m/s, 5.1 m/s]
[-2 m, 4 m] * [1 s, 3 s] -> [-6 m*s, 12 m*s]
```

### Dates, Relative Time & Business Calendars

Abacus includes a comprehensive date/time calculation engine:

```text
// Date arithmetic & formatting
07-08-2026 + 3 weeks               -> 28-08-2026
07-08-2026 + 5 business days       -> 14-08-2026
10-08-2026 - 1 business day        -> 07-08-2026
workdays(07-08-2026, 14-08-2026)   -> 5 workdays

// Relative time & natural language
now + 2 hours
today at 12 to 1
last thursday at 3pm + 2 weeks
3 days ago
in 45 minutes

// Timezones & AM/PM
15:30:00 EST to PST                -> 12:30:00 PST
3:30 PM + 45 minutes               -> 04:15:00 PM
```

### Statistics & Probability Distributions

```text
// Descriptive statistics on ranges or arguments
mean(1..10)                        -> 5.5
median(10, 20, 30, 40, 50)         -> 30
std(1..9..2)                       -> 2.8284271247461903
iqr(1..100)                        -> 50

// Distributions
normpdf(0, 0, 1)                   -> 0.3989422804014327
normcdf(1.96, 0, 1)                -> 0.9750021048517796
invnorm(0.975, 0, 1)               -> 1.959963984540054
poissonpmf(3, 2)                   -> 0.22404180765538775
poissonpmf(200, 180)               -> (evaluated in log-space without overflow)
```

### Hypothesis Testing & Confidence Intervals

TI-84-style hypothesis tests return structured hashes containing test statistics, p-values, degrees of freedom, and sample estimates:

```text
// Z-Test on sample data
ZTest(10 m, 2 m, 12 m, 14 m, 10 m, 16 m)

// Output Hash:
{
    df: 3,
    mean: 13 m,
    p_value: 0.0026997960632601965,
    z: 3
}

// Access individual properties via dot notation:
ZTest(10 m, 2 m, 12 m, 14 m, 10 m, 16 m).p_value
```

Supported tests and intervals:

- **Tests**: `ZTest`, `TTest`, `2-SampZTest`, `2-SampTTest`, `1-PropZTest`, `2-PropZTest`, `Chi2Test`
- **Confidence Intervals**: `ZInterval`, `TInterval`, `1-PropZInt`, `2-SampZInt`, `2-SampTInt`, `2-PropZInt`

### Dimension-Aware Linear Regression

`linreg` calculates linear regression while maintaining physical dimensions for slopes, intercepts, standard errors, and predictions:

```text
linreg(1 s, 2 s, 3 s, 4 s, 10 m, 20 m, 30 m, 40 m)

// Output:
{
    intercept: 0 m,
    mean_x: 2.5 s,
    mean_y: 25 m,
    r: 1,
    r2: 1,
    se: 0 m,
    slope: 10 m/s
}

// Extrapolate directly:
linreg(1 s, 2 s, 3 s, 4 s, 10 m, 20 m, 30 m, 40 m).slope * 10 s -> 100 m
```

---

## Cargo Features

Abacus is modular and allows disabling features you do not need:

```toml
[dependencies]
abacus = { version = "0.1", default-features = false, features = ["units"] }
```

| Feature         | Description                                                                               | Default |
| :-------------- | :---------------------------------------------------------------------------------------- | :------ |
| `units`         | Full physical unit registry and conversions                                               | **Yes** |
| `stats`         | Descriptive statistics, dispersion, and linear regression                                 | **Yes** |
| `distributions` | Continuous & discrete probability distributions                                           | **Yes** |
| `date`          | Date parsing, relative time, and business day arithmetic                                  | **Yes** |
| `financial`     | Financial functions (`pmt`, `npv`, `irr`, `fv`, `pv`)                                     | **Yes** |
| `repl`          | Interactive CLI REPL with command history and syntax highlighting                         | **Yes** |
| `serde`         | Enables `Serialize` and `Deserialize` on core types (`Value`, `Interval`, `Date`, `Hash`) | No      |

---

## Benchmarks

Run the Criterion benchmark suite:

```sh
cargo bench --bench eval_bench
```

Or execute benchmark smoke tests:

```sh
cargo bench --bench eval_bench -- --test
```

---

## REPL

Launch the interactive REPL:

```sh
cargo run --bin repl
```

The REPL supports persistent cross-platform history, ANSI syntax formatting in TTY environments, and live expression inspection.

---

## Testing & Quality

Run the test suite and doctests:

```sh
# Run all unit and integration tests
cargo test --all-targets --all-features

# Run executable doctests
cargo test --doc

# Run clippy
cargo clippy --all-targets --all-features
```

## License

MIT
