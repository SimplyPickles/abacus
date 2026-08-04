# Abacus

**Abacus** is a fast, zero-dependency, unit-aware mathematical evaluation engine written in Rust. It tracks physical dimensions across arithmetic operations, parses mathematical expressions with natural implicit multiplication and ranges, and includes a rich suite of statistical functions and probability distributions.

---

## ✨ Features

- 🧮 **Pratt Parser Engine**: Evaluates expressions using Pratt parsing with operator precedence, parenthesized grouping, unary/binary/postfix operations, and unit conversions (`as`, `in`, `to`).
- ✖️ **Implicit Multiplication (Juxtaposition)**: Supports natural mathematical expressions like `5(2 + 3)`, `(2 + 3)(4 + 5)`, and `2 sqrt(9 m^2)`.
- 📏 **Dimension-Aware & Fractional Scaling**: Tracks physical dimensions across `+`, `-`, `*`, `/`, `^`, and `sqrt` using a 8-dimensional space (`[length, mass, time, current, temp, amount, luminous, info]`) with fractional exponent support.
- 🔄 **Dimensionless & Bare Unit Promotion**:
  - `5 m + 5` $\to$ `10 m` and `5 cm + 5` $\to$ `10 cm` (dimensionless numbers automatically adopt adjacent units in addition/subtraction).
  - `1 as inches` $\to$ `1 in` (attaches units to unitless values).
  - Bare unit expressions like `1m/m` or `5 km / m` evaluate cleanly to `1` or `5000`.
- 💡 **Automatic Derived SI Unit Reduction**: Compound unit calculations automatically reduce to named derived SI units at the end of evaluation (e.g. `10 N * 5 m` $\to$ `50 J`, `100 W * 5 s` $\to$ `500 J`, `12 V * 2 A` $\to$ `24 W`, `10 N / 2 m^2` $\to$ `5 Pa`, `5 C / 2 s` $\to$ `2.5 A`).
- 📊 **Statistical Functions & Range Step Expansion**:
  - Functions accept discrete arguments, standard ranges (`1..5`, `1 m .. 5 m`), or explicit **range step syntax** (`1..9..2`, `0 m .. 10 m .. 2 m`, `0 km .. 1 km .. 250 m`).
  - Includes `sum`, `mean`, `median`, `mode`, `range`, `var`, `std`, `quantile`, `percentile`, `iqr`, and `corr` (Pearson correlation).
- 🎲 **Combinatorics**: `!` (factorial operator), `factorial(n)`, `nCr(n, r)` / `comb(n, r)` (combinations), `nPr(n, r)` / `perm(n, r)` (permutations).
- ➗ **Unit-Aware Modulo & Math Helpers**:
  - `%` operator and `mod(a, b)` / `modulo(a, b)` (e.g. `10 m % 3 m` $\to$ `1 m`, `10 cm % 3` $\to$ `1 cm`).
  - `clamp(x, min, max)` (preserves unit, e.g. `clamp(15 m, 0 m, 10 m)` $\to$ `10 m`).
  - `gcd(a, b)` and `lcm(a, b)`.
- 📐 **Trigonometry, Logarithms & Math Library**:
  - **Trigonometric & Hyperbolic**: `sin`, `cos`, `tan`, `asin`, `acos`, `atan`, `atan2`, `sinh`, `cosh`, `tanh`, `asinh`, `acosh`, `atanh` (supports angle units like `rad`, `deg`, `°`, `turn`, `arcmin`, `arcsec`).
  - **Logarithmic & Exponential**: `ln`, `log10`, `log2`, `log(x, base)`, `exp`.
  - **Rounding & General Math**: `abs`, `floor`, `ceil`, `round`, `sign` (preserves unit, e.g. `floor(5.7 m) = 5 m`).
- 🌌 **Extensive Unit Registry**:
  - SI Base & Derived Units (`m`, `kg`, `s`, `N`, `J`, `W`, `Pa`, `Hz`, `V`, `Ω`, `F`, `Wb`, `T`, `H`, etc.)
  - Exponent units (`m^2`, `m^3`, `cm^3`, `ft^3`) dynamically resolved.
  - Imperial & US Customary (`in`, `ft`, `yd`, `mi`, `lb`, `oz`, `us_gal`, `bbl`, `fl oz`)
  - Storage units with decimal (`kB`, `MB`) and binary (`KiB`, `MiB`) prefixes
  - Angles (`rad`, `deg`, `turn`, `arcmin`, `arcsec`)
  - Astronomical (`au`, `ly`, `pc`, `solar_mass`), Nautical, CGS Physics (`eV`, `Å`, `bar`, `gauss`), Typography, Historical, and Humorous (`smoot`, `fortnight`, `attoparsec`) units.
  - Affine unit protection for temperature (`°C`, `°F`, `K`).

---

## 🚀 Quickstart

Add `abacus` to your `Cargo.toml`:

```rust
use abacus::eval;

fn main() {
    // Basic unit arithmetic
    println!("{}", eval("5 m + 3 m").unwrap());             // 8 m
    println!("{}", eval("(5 m + 20 cm) as m").unwrap());     // 5.2 m
    println!("{}", eval("5 km to m").unwrap());              // 5000 m

    // Implicit multiplication & dimensionless promotion
    println!("{}", eval("5(2 + 3)").unwrap());              // 25
    println!("{}", eval("5 m + 5").unwrap());               // 10 m
    println!("{}", eval("5 cm + 5").unwrap());              // 10 cm
    println!("{}", eval("1 as inches").unwrap());           // 1 in

    // Ranges & Statistics
    println!("{}", eval("sum(1 m .. 5 m)").unwrap());        // 15 m
    println!("{}", eval("mean(10 m .. 30 m)").unwrap());     // 20 m
    println!("{}", eval("corr(1..5, 2..6)").unwrap());       // 1

    // Probability Distributions & Inverse CDFs
    println!("{}", eval("normcdf(70 kg, 65 kg, 5 kg)").unwrap());   // 0.8413447
    println!("{}", eval("invnorm(0.975, 100 kg, 15 kg)").unwrap()); // 129.38573 kg
    println!("{}", eval("invt(0.975, 10)").unwrap());               // 2.2281388
}
```

---

## 📖 Evaluation Examples

### 1. Basic Unit Arithmetic & Conversions

```text
Expression                          Result
----------------------------------  ----------------------
5 m + 3 m                           = 8 m
(5 m + 20 cm) as m                  = 5.2 m
sqrt(9 m^2)                         = 3 m
1m/m                                = 1
5 km / m                            = 5000
5 km to m                           = 5000 m
1 m in inches                       = 39.37007874 in
```

### 2. Implicit Multiplication & Dimensionless Promotion

```text
Expression                          Result
----------------------------------  ----------------------
5(2 + 3)                            = 25
(2 + 3)(4 + 5)                      = 45
2 sqrt(9 m^2)                       = 6 m
5 m + 5                             = 10 m
5 cm + 5                            = 10 cm
10 cm - 3                           = 7 cm
1 as inches                         = 1 in
```

### 3. Statistics & Ranges

```text
Expression                          Result
----------------------------------  ----------------------
sum(1 m .. 5 m)                     = 15 m
mean(10 m .. 30 m)                  = 20 m
median(1 m, 10 m, 5 m, 20 m)        = 7.5 m
mode(2 m, 5 m, 2 m, 8 m)            = 2 m
range(1 m .. 10 m)                  = 9 m
quantile(1 m .. 5 m, 0.75)          = 4 m
percentile(1 m .. 5 m, 75)          = 4 m
iqr(1 m .. 5 m)                     = 2 m
corr(1..5, 2..6)                    = 1
var(1 m .. 5 m)                     = 2.5 m^2
std(1 m .. 5 m)                     = 1.5811388 m
```

### 4. Distributions & Inverse CDFs

```text
Expression                          Result
----------------------------------  ----------------------
binompdf(10, 0.5, 5)                = 0.24609375
binomcdf(10, 0.5, 5)                = 0.623046875
geompdf(0.5, 3)                     = 0.125
poissoncdf(3, 2)                    = 0.42319008
normpdf(0)                          = 0.39894228
normcdf(70 kg, 65 kg, 5 kg)         = 0.8413447
invnorm(0.975)                      = 1.9590489
invnorm(0.975, 100 kg, 15 kg)       = 129.385734 kg
tcdf(10, 2.228)                     = 0.9749941
invt(0.975, 10)                     = 2.2281388
chisqcdf(10, 18.307)                = 0.9499994
invchisq(0.95, 10)                  = 18.307038
fcdf(5, 10, 3.33)                   = 0.9501687
expcdf(0.5, 2)                      = 0.63212055
invexp(0.63212, 0.5)                = 1.99999696
unifcdf(0, 10, 5)                   = 0.5
invunif(0.5, 0, 10)                 = 5
```

---

## 🛠️ Testing & Verification

Run the test suite:

```sh
cargo test
```

All **87 integration and unit tests** pass, covering Pratt parser precedence, range expansion, unit cancellation, fractional dimension square roots, statistical functions, probability distributions, inverse CDFs, and implicit multiplication.

Run the demonstration CLI:

```sh
cargo run
```

---

## 📜 License

MIT License. Free for open-source and commercial use.
