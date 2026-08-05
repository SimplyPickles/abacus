use abacus::{Abacus, AbacusError, EvalResult};

fn eval(expr: &str) -> Result<EvalResult, AbacusError> {
    Abacus::standard().eval(expr)
}

// ── Basic interval construction ──

#[test]
fn test_basic_interval_construction() {
    // Verifies [lo, hi] syntax produces an interval with correct endpoints
    let result = eval("[1 m, 5 m]").unwrap();
    assert_eq!(result.to_display(), "[1 m, 5 m]");
}

#[test]
fn test_interval_auto_normalizes_order() {
    // Verifies that [hi, lo] is auto-normalized to [lo, hi]
    let result = eval("[10 m, 2 m]").unwrap();
    assert_eq!(result.to_display(), "[2 m, 10 m]");
}

#[test]
fn test_dimensionless_interval() {
    // Verifies creating intervals with plain numbers (no units)
    let result = eval("[3, 7]").unwrap();
    assert_eq!(result.to_display(), "[3, 7]");
}

// ── Interval + Interval arithmetic ──

#[test]
fn test_interval_addition() {
    // [1 m, 3 m] + [2 m, 4 m] = [3 m, 7 m]
    let result = eval("[1 m, 3 m] + [2 m, 4 m]").unwrap();
    assert_eq!(result.to_display(), "[3 m, 7 m]");
}

#[test]
fn test_interval_subtraction() {
    // [5 m, 10 m] - [1 m, 3 m] = [2 m, 9 m] (min: 5-3=2, max: 10-1=9)
    let result = eval("[5 m, 10 m] - [1 m, 3 m]").unwrap();
    assert_eq!(result.to_display(), "[2 m, 9 m]");
}

#[test]
fn test_interval_multiplication() {
    // [2, 3] * [4, 5] = [8, 15] (all corners positive)
    let result = eval("[2, 3] * [4, 5]").unwrap();
    assert_eq!(result.to_display(), "[8, 15]");
}

#[test]
fn test_interval_division_the_flagship_example() {
    // The headline feature: [9.8 m, 10.2 m] / [1.9 s, 2.1 s]
    // Corners: 9.8/1.9≈5.157, 9.8/2.1≈4.666, 10.2/1.9≈5.368, 10.2/2.1≈4.857
    // Result: [4.666... m/s, 5.368... m/s]
    let result = eval("[9.8 m, 10.2 m] / [1.9 s, 2.1 s]").unwrap();
    let display = result.to_display();
    assert!(
        display.starts_with("["),
        "Expected interval display, got: {}",
        display
    );
    assert!(
        display.contains("m/s"),
        "Expected m/s units, got: {}",
        display
    );

    // Extract numerical bounds and verify
    if let EvalResult::Interval(interval) = result {
        let lo_val = (interval.lo.canonical - interval.lo.unit.offset) / interval.lo.unit.scalar;
        let hi_val = (interval.hi.canonical - interval.hi.unit.offset) / interval.hi.unit.scalar;
        assert!(
            (lo_val - 4.666666).abs() < 0.01,
            "lo should be ~4.666, got {}",
            lo_val
        );
        assert!(
            (hi_val - 5.368421).abs() < 0.01,
            "hi should be ~5.368, got {}",
            hi_val
        );
    } else {
        panic!("Expected interval result");
    }
}

// ── Scalar + Interval mixed arithmetic ──

#[test]
fn test_scalar_plus_interval() {
    // 5 m + [1 m, 3 m] = [6 m, 8 m]
    let result = eval("5 m + [1 m, 3 m]").unwrap();
    assert_eq!(result.to_display(), "[6 m, 8 m]");
}

#[test]
fn test_interval_plus_scalar() {
    // [1 m, 3 m] + 5 m = [6 m, 8 m]
    let result = eval("[1 m, 3 m] + 5 m").unwrap();
    assert_eq!(result.to_display(), "[6 m, 8 m]");
}

#[test]
fn test_interval_times_scalar() {
    // [2 m, 4 m] * 3 = [6 m, 12 m]
    let result = eval("[2 m, 4 m] * 3").unwrap();
    assert_eq!(result.to_display(), "[6 m, 12 m]");
}

#[test]
fn test_scalar_times_interval() {
    // 3 * [2 m, 4 m] = [6 m, 12 m]
    let result = eval("3 * [2 m, 4 m]").unwrap();
    assert_eq!(result.to_display(), "[6 m, 12 m]");
}

// ── Interval with unit conversion ──

#[test]
fn test_interval_unit_conversion() {
    // [1 km, 2 km] to m = [1000 m, 2000 m]
    let result = eval("[1 km, 2 km] to m").unwrap();
    assert_eq!(result.to_display(), "[1000 m, 2000 m]");
}

#[test]
fn test_interval_unit_conversion_mixed_units() {
    // [500 m, 2 km] to m = [500 m, 2000 m]
    let result = eval("[500 m, 2 km] to m").unwrap();
    assert_eq!(result.to_display(), "[500 m, 2000 m]");
}

// ── Interval in grouped expressions ──

#[test]
fn test_interval_in_parentheses() {
    // ([1, 3] + [2, 4]) * 2 = [3, 7] * 2 = [6, 14]
    let result = eval("([1, 3] + [2, 4]) * 2").unwrap();
    assert_eq!(result.to_display(), "[6, 14]");
}

// ── Implicit multiplication with intervals ──

#[test]
fn test_implicit_multiplication_with_interval() {
    // 5 [1, 3] = 5 * [1, 3] = [5, 15]
    let result = eval("5 [1, 3]").unwrap();
    assert_eq!(result.to_display(), "[5, 15]");
}

// ── Negative and zero-crossing intervals ──

#[test]
fn test_interval_crossing_zero_multiplication() {
    // [-2, 3] * [1, 4] = corners: -2*1=-2, -2*4=-8, 3*1=3, 3*4=12 → [-8, 12]
    let result = eval("[-2, 3] * [1, 4]").unwrap();
    assert_eq!(result.to_display(), "[-8, 12]");
}

#[test]
fn test_negative_interval() {
    // [-5, -1] + [2, 3] = [-3, 2]
    let result = eval("[-5, -1] + [2, 3]").unwrap();
    assert_eq!(result.to_display(), "[-3, 2]");
}

// ── Derived unit reduction with intervals ──

#[test]
fn test_interval_derived_unit_reduction() {
    // Force × distance → Joules: [10 N, 20 N] * [2 m, 5 m] = [20 J, 100 J]
    let result = eval("[10 N, 20 N] * [2 m, 5 m]").unwrap();
    assert_eq!(result.to_display(), "[20 J, 100 J]");
}

// ── Unary operators on intervals ──

#[test]
fn test_unary_negation_on_interval() {
    // -[2 m, 5 m] = [-5 m, -2 m]
    let result = eval("-[2 m, 5 m]").unwrap();
    assert_eq!(result.to_display(), "[-5 m, -2 m]");
}

// ── Error cases ──

#[test]
fn test_interval_in_function_errors() {
    // Intervals cannot be passed as function arguments
    assert!(matches!(
        eval("mean([1, 3])"),
        Err(AbacusError::IntervalInFunction)
    ));
}

#[test]
fn test_incompatible_interval_dimensions_error() {
    // [1 m, 5 s] should error — incompatible dimensions
    assert!(matches!(
        eval("[1 m, 5 s]"),
        Err(AbacusError::IncompatibleDimensions)
    ));
}

#[test]
fn test_unclosed_bracket_error() {
    // [1, 3 without closing ] should error
    assert!(eval("[1, 3").is_err());
}

#[test]
fn test_interval_missing_comma_error() {
    // [1 3] without comma should error
    assert!(eval("[1 3]").is_err());
}
