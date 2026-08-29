# Abacus

**Abacus** is a zero-dependency, unit-aware mathematical evaluation engine written in Rust. It tracks physical dimensions across arithmetic operations, parses mathematical expressions with implicit multiplication and ranges, and includes statistical functions, probability distributions, physical interval arithmetic, confidence intervals, and dimension-aware linear regression.

---

## Features

- **Pratt Parser Engine**: Evaluates expressions using Pratt parsing with operator precedence, parenthesized grouping, unary/binary/postfix operations, dot property access (`.field`), and unit conversions (`as`, `in`, `to`).
- **Implicit Multiplication (Juxtaposition)**: Supports natural mathematical expressions like `5(2 + 3)`, `(2 + 3)(4 + 5)`, and `2 sqrt(9 m^2)`.
- **Physical Interval Arithmetic**: Compute worst/best-case physical boundaries using bracket syntax `[lo, hi]` (e.g., `[9.8 m, 10.2 m] / [1.9 s, 2.1 s]` $\to$ `[4.666 m/s, 5.368 m/s]`).
- **Dimension-Aware Linear Regression**: Compute linear regression with physical unit reduction (`linreg(x_data, y_data)` $\to$ returns a `Hash` with slope, intercept, $R^2$, $r$, standard error, and predictions).
- **Dot Property Access**: Extract properties directly on Hash-returning functions in expressions (e.g., `linreg(...).intercept`, `linreg(...).slope * 5 s`).
- **TI-84 Inferential Statistics**: Confidence interval menu functions (`TInterval`, `ZInterval`, `1-PropZInt`, `2-SampTInt`, `2-SampZInt`, `2-PropZInt`).
- **Dimension-Aware & Fractional Scaling**: Tracks physical dimensions across `+`, `-`, `*`, `/`, `^`, and `sqrt` using an 8-dimensional space (`[length, mass, time, current, temp, amount, luminous, info]`) with fractional exponent support.
- **Dimensionless & Bare Unit Promotion**:
  - `5 m + 5` $\to$ `10 m` and `5 cm + 5` $\to$ `10 cm` (dimensionless numbers automatically adopt adjacent units in addition/subtraction).
  - `1 as inches` $\to$ `1 in` (attaches units to unitless values).
  - Bare unit expressions like `1m/m` or `5 km / m` evaluate to `1` or `5000`.
- **Automatic Derived SI Unit Reduction**: Compound unit calculations automatically reduce to named derived SI units at the end of evaluation (e.g., `10 N * 5 m` $\to$ `50 J`, `100 W * 5 s` $\to$ `500 J`, `12 V * 2 A` $\to$ `24 W`), while respecting explicit user conversion targets (e.g., `50 J to N*m` $\to$ `50 N*m`).
- **Statistical Functions & Range Step Expansion**:
  - Functions accept discrete arguments, standard ranges (`1..5`, `1 m .. 5 m`), or explicit range step syntax (`1..9..2`, `0 m .. 10 m .. 2 m`, `0 km .. 1 km .. 250 m`).
  - **Descriptive & Averages**: `sum`, `mean`, `geomean`, `harmean`, `median`, `mode`, `min`, `max`, `range`, `mad`, `rms`.
  - **Variance & Dispersion**: `var` / `var_s`, `var_p`, `std` / `std_s`, `std_p`, `quantile`, `percentile`, `iqr`.
  - **Bivariate & Shape**: `cov` / `cov_s`, `cov_p`, `corr`, `skew` / `skewness`, `kurt` / `kurtosis`, `zscore` / `standardize`.
- **Combinatorics**: `!` (factorial operator), `factorial(n)`, `nCr(n, r)` / `comb(n, r)`, `nPr(n, r)` / `perm(n, r)`.
- **Financial & TVM Functions**: `pmt`, `fv`, `pv`, `npv`, `irr`, `compound`.
- **Trigonometry, Logarithms & Math Library**:
  - **Trigonometric & Hyperbolic**: `sin`, `cos`, `tan`, `asin`, `acos`, `atan`, `atan2`, `sinh`, `cosh`, `tanh`, `asinh`, `acosh`, `atanh` (supports angle units like `rad`, `deg`, `°`, `turn`, `arcmin`, `arcsec`).
  - **Logarithmic & Exponential**: `ln`, `log10`, `log2`, `log(x, base)`, `exp`.
  - **Rounding & General Math**: `abs`, `floor`, `ceil`, `round`, `sign` (preserves unit, e.g., `floor(5.7 m) = 5 m`).
- **Unit Registry**:
  - SI Base & Derived Units (`m`, `kg`, `s`, `N`, `J`, `W`, `Pa`, `Hz`, `V`, `Ω`, `F`, `Wb`, `T`, `H`, etc.)
  - Exponent units (`m^2`, `m^3`, `cm^3`, `ft^3`) dynamically resolved.
  - Imperial & US Customary (`in`, `ft`, `yd`, `mi`, `lb`, `oz`, `us_gal`, `bbl`, `fl oz`)
  - Storage units with decimal (`kB`, `MB`) and binary (`KiB`, `MiB`) prefixes
  - Angles (`rad`, `deg`, `turn`, `arcmin`, `arcsec`)
  - Astronomical (`au`, `ly`, `pc`, `solar_mass`), Nautical, CGS Physics (`eV`, `Å`, `bar`, `gauss`), Typography, Historical, and Humorous (`smoot`, `fortnight`, `attoparsec`) units.
  - Affine unit protection for temperature (`°C`, `°F`, `K`).

---

## Quickstart

Add `abacus` to your `Cargo.toml`:

```rust
use abacus::Abacus;

fn main() {
    let calc = Abacus::standard();

    // Basic unit arithmetic & derived SI reduction
    println!("{}", calc.eval("10 N * 5 m").unwrap());              // 50 J
    println!("{}", calc.eval("100 km / 2 h to m/s").unwrap());    // 13.88888888888889 m/s

    // Physical Interval Arithmetic
    println!("{}", calc.eval("[9.8 m, 10.2 m] / [1.9 s, 2.1 s]").unwrap()); // [4.666 m/s, 5.368 m/s]

    // TI-84 Confidence Intervals
    println!("{}", calc.eval("TInterval(10 m, 12 m, 11 m, 14 m)").unwrap()); // [9.032 m, 14.468 m]

    // Linear Regression & Dot Property Access
    println!("{}", calc.eval("linreg(1 s, 2 s, 3 s, 4 s, 10 m, 20 m, 30 m, 40 m)").unwrap());
    // -> { intercept: 0 m, mean_x: 2.5 s, mean_y: 25 m, r: 1, r2: 1, se: 0 m, slope: 10 m/s }

    println!("{}", calc.eval("linreg(1 s, 2 s, 3 s, 4 s, 10 m, 20 m, 30 m, 40 m).slope * 5 s").unwrap());
    // -> 50 m
}
