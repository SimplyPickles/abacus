use abacus::{Abacus, AbacusError, AngleMode, IntervalStyle, Notation};

#[test]
fn test_angle_mode_degrees() {
    let calc = Abacus::standard().with_angle_mode(AngleMode::Degrees);

    let sin_90 = calc.eval("sin(90)").unwrap();
    assert!((sin_90.into_scalar().unwrap().canonical - 1.0).abs() < 1e-10);

    let cos_180 = calc.eval("cos(180)").unwrap();
    assert!((cos_180.into_scalar().unwrap().canonical - (-1.0)).abs() < 1e-10);

    let tan_45 = calc.eval("tan(45)").unwrap();
    assert!((tan_45.into_scalar().unwrap().canonical - 1.0).abs() < 1e-10);

    // Inverse trig converts back to degrees
    let asin_1 = calc.eval("asin(1)").unwrap();
    assert_eq!(asin_1.to_display(), "90 deg");

    let acos_0 = calc.eval("acos(0)").unwrap();
    assert_eq!(acos_0.to_display(), "90 deg");

    let atan_1 = calc.eval("atan(1)").unwrap();
    assert_eq!(atan_1.to_display(), "45 deg");

    // Explicit angle unit overrides bare interpretation
    let sin_explicit = calc.eval("sin(90 deg)").unwrap();
    assert!((sin_explicit.into_scalar().unwrap().canonical - 1.0).abs() < 1e-10);
}

#[test]
fn test_angle_mode_gradians() {
    let calc = Abacus::standard().with_angle_mode(AngleMode::Gradians);

    let sin_100 = calc.eval("sin(100)").unwrap();
    assert!((sin_100.into_scalar().unwrap().canonical - 1.0).abs() < 1e-10);

    let cos_200 = calc.eval("cos(200)").unwrap();
    assert!((cos_200.into_scalar().unwrap().canonical - (-1.0)).abs() < 1e-10);

    let tan_50 = calc.eval("tan(50)").unwrap();
    assert!((tan_50.into_scalar().unwrap().canonical - 1.0).abs() < 1e-10);

    let asin_1 = calc.eval("asin(1)").unwrap();
    assert_eq!(asin_1.to_display(), "100 grad");
}

#[test]
fn test_angle_mode_radians_default() {
    let calc = Abacus::standard();
    assert_eq!(calc.angle_mode, AngleMode::Radians);

    let asin_1 = calc.eval("asin(1)").unwrap();
    assert!(asin_1.to_display().contains("rad"));
}

#[test]
fn test_strict_dimension_checking() {
    let loose = Abacus::standard();
    assert!(!loose.strict_dimensions);

    // In loose mode, dimensionless promotion succeeds
    let promoted = loose.eval("5 m + 5").unwrap();
    assert_eq!(promoted.to_display(), "10 m");

    let promoted_interval = loose.eval("[1 m, 2]").unwrap();
    assert_eq!(promoted_interval.to_display(), "[1 m, 2 m]");

    // In strict mode, mixing dimensions fails with IncompatibleDimensions
    let strict = Abacus::standard().with_strict_dimensions(true);
    assert!(strict.strict_dimensions);

    assert!(matches!(
        strict.eval("5 m + 5"),
        Err(AbacusError::IncompatibleDimensions)
    ));
    assert!(matches!(
        strict.eval("5 + 5 m"),
        Err(AbacusError::IncompatibleDimensions)
    ));
    assert!(matches!(
        strict.eval("5 m - 2"),
        Err(AbacusError::IncompatibleDimensions)
    ));
    assert!(matches!(
        strict.eval("[1 m, 2]"),
        Err(AbacusError::IncompatibleDimensions)
    ));
    assert!(matches!(
        strict.eval("1 m .. 2"),
        Err(AbacusError::IncompatibleDimensions)
    ));

    // Percentage arithmetic is still allowed because % modifies the base quantity
    let pct = strict.eval("100 m + 10%").unwrap();
    assert_eq!(pct.to_display(), "110 m");
}

#[test]
fn test_decimal_places_configuration() {
    let calc = Abacus::standard().with_decimal_places(2);
    assert_eq!(calc.decimal_places, Some(2));

    // 10 / 3 = 3.3333... -> 3.33
    let res = calc.eval("10 / 3").unwrap();
    assert_eq!(res.to_display(), "3.33");

    // 1.23456 m -> 1.23 m
    let res_m = calc.eval("1.23456 m").unwrap();
    assert_eq!(res_m.to_display(), "1.23 m");

    // Trailing zero formatting with format_result
    let res_five = calc.eval("5").unwrap();
    assert_eq!(calc.format_result(&res_five), "5.00");
}

#[test]
fn test_interval_style_configuration() {
    let default_calc = Abacus::standard();
    let res = default_calc.eval("[1 m, 5 m] + 2 m").unwrap();
    assert_eq!(res.to_display(), "[3 m, 7 m]");

    let range_calc = Abacus::standard().with_interval_style(IntervalStyle::Range);
    let res_range = range_calc.eval("[1 m, 5 m] + 2 m").unwrap();
    assert_eq!(res_range.to_display(), "3 m..7 m");
}

#[test]
fn test_notation_modes() {
    let calc_eng = Abacus::standard().with_notation(Notation::Engineering);
    assert_eq!(calc_eng.notation, Notation::Engineering);

    let res1 = calc_eng.eval("45000").unwrap();
    assert_eq!(calc_eng.format_result(&res1), "45e3");

    let res2 = calc_eng.eval("1200000").unwrap();
    assert_eq!(calc_eng.format_result(&res2), "1.2e6");

    let res3 = calc_eng.eval("0.045").unwrap();
    assert_eq!(calc_eng.format_result(&res3), "45e-3");

    let calc_sci = Abacus::standard().with_notation(Notation::Scientific);
    let res_sci = calc_sci.eval("1200000").unwrap();
    assert!(calc_sci.format_result(&res_sci).contains("1.2e6"));
}

#[test]
fn test_in_place_mutator_methods() {
    let mut calc = Abacus::standard();

    calc.set_angle(AngleMode::Degrees);
    assert_eq!(calc.angle_mode, AngleMode::Degrees);

    calc.set_strict_dims(true);
    assert!(calc.strict_dimensions);

    calc.set_dec_places(Some(3));
    assert_eq!(calc.decimal_places, Some(3));

    calc.set_int_style(Some(IntervalStyle::Range));
    assert_eq!(calc.default_interval_style, Some(IntervalStyle::Range));

    calc.set_note_mode(Notation::Engineering);
    assert_eq!(calc.notation, Notation::Engineering);
}
