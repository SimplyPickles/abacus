use abacus::{eval, AbacusError};

#[test]
fn test_fractional_exponent_dimensions() {
    // Square root of m^4 / s^2 is m^2/s
    let res = eval("sqrt(9 m^4 / s^2)").unwrap();
    assert_eq!(res.to_display(), "3 m^2/s");

    // Power with fractional exponents
    let res2 = eval("(16 m^2)^0.5").unwrap();
    assert_eq!(res2.canonical, 4.0);

    // Negative exponent
    let res3 = eval("1 / (4 s^2)^0.5").unwrap();
    assert_eq!(res3.canonical, 0.5);
}

#[test]
fn test_mixed_prefix_and_unit_conversion_chains() {
    // Mixed metric prefix addition: km + m + cm -> m
    let res = eval("(1 km + 500 m + 2500 cm) as m").unwrap();
    assert_eq!(res.to_display(), "1525 m");

    // Storage unit conversions & binary prefixes
    let res2 = eval("1 TiB to GiB").unwrap();
    assert_eq!(res2.to_display(), "1024 GiB");

    let res3 = eval("(1 MB + 500 kB) as kB").unwrap();
    assert_eq!(res3.to_display(), "1500 kB");

    let res4 = eval("1 GiB / 1 MiB").unwrap();
    assert_eq!(res4.canonical, 1024.0);
}

#[test]
fn test_affine_temperature_conversions_and_protections() {
    // Valid conversions
    assert_eq!(eval("0 °C as °F").unwrap().to_display(), "32 °F");
    assert_eq!(eval("100 °C as K").unwrap().to_display(), "373.15 K");
    assert_eq!(eval("98.6 °F as °C").unwrap().to_display(), "37 °C");

    // Disallowed affine arithmetic (e.g. adding two affine temperatures directly)
    assert!(matches!(
        eval("20 °C + 10 °C"),
        Err(AbacusError::AffineUnitOperation(_))
    ));
    assert!(matches!(
        eval("++ 20 °C"),
        Err(AbacusError::AffineUnitOperation(_))
    ));
}

#[test]
fn test_unusual_niche_and_humorous_units() {
    // Smoot
    let smoot = eval("1 smoot to inches").unwrap();
    assert_eq!(smoot.to_display(), "67 in");

    // Fortnight
    let fortnight = eval("1 fortnight to h").unwrap();
    assert_eq!(fortnight.to_display(), "336 h");

    // Typography
    let pica = eval("1 pica to point").unwrap();
    assert_eq!(pica.to_display(), "12 pt_type");
}

#[test]
fn test_cgs_and_astronomical_physics_conversions() {
    // Astronomical: pc to ly
    let pc = eval("1 pc to ly").unwrap();
    let ly_value = pc.canonical / 9_460_730_472_580_800.0;
    assert!((ly_value - 3.2615637).abs() < 1e-4);

    // AU to km
    let au = eval("1 au to km").unwrap();
    assert_eq!(au.to_display(), "149597870.7 km");

    // CGS Physics: bar to Pa
    let bar = eval("1 bar to Pa").unwrap();
    assert_eq!(bar.to_display(), "100000 Pa");
}

#[test]
fn test_complex_implicit_multiplication() {
    // 2(3 + 4)(5 + 6) = 2 * 7 * 11 = 154
    assert_eq!(eval("2(3 + 4)(5 + 6)").unwrap().to_display(), "154");

    // Juxtaposition with functions: 2 sqrt(16 m^2) 3 = 2 * 4 m * 3 = 24 m
    assert_eq!(eval("2 sqrt(16 m^2) 3").unwrap().to_display(), "24 m");

    // Double parentheses juxtaposition
    assert_eq!(eval("2(3)4").unwrap().to_display(), "24");

    // Complex nested parentheses
    assert_eq!(eval("((2 + 3) * (4 + 5))^2").unwrap().to_display(), "2025");
}

#[test]
fn test_statistical_dispersion_and_multi_unit_ranges() {
    // Mean of mixed compatible units: 1 m, 100 cm, 2000 mm -> 4/3 m = 1.3333333333333333 m
    let m = eval("mean(1 m, 100 cm, 2000 mm)").unwrap();
    assert!((m.canonical - 1.3333333333333333).abs() < 1e-6);

    // Quantile
    let q = eval("quantile(1 m .. 9 m, 0.5)").unwrap();
    assert_eq!(q.to_display(), "5 m");

    // Pearson Correlation
    let corr_pos = eval("corr(1..5, 10..14)").unwrap().canonical;
    assert!((corr_pos - 1.0).abs() < 1e-6);

    let corr_neg = eval("corr(1..5, 5..1)").unwrap().canonical;
    assert!((corr_neg - (-1.0)).abs() < 1e-6);
}

#[test]
fn test_custom_range_step_expansion_edge_cases() {
    // Step expansion with units: 0 m .. 10 m .. 2.5 m -> sum = 0 + 2.5 + 5 + 7.5 + 10 = 25 m
    assert_eq!(eval("sum(0 m .. 10 m .. 2.5 m)").unwrap().to_display(), "25 m");

    // Range with step in reverse direction
    assert_eq!(eval("sum(10 .. 0 .. -2)").unwrap().to_display(), "30");
}

#[test]
fn test_advanced_trigonometric_angle_units() {
    assert_eq!(eval("sin(90 deg)").unwrap().to_display(), "1");
    assert_eq!(eval("cos(180 deg)").unwrap().to_display(), "-1");
    assert_eq!(eval("tan(45 deg)").unwrap().to_display(), "1");
    assert_eq!(eval("asin(1) to deg").unwrap().to_display(), "90 deg");

    let atan_res = eval("atan2(1 m, 1 m) to deg").unwrap();
    assert_eq!(atan_res.to_display(), "45 deg");
}

#[test]
fn test_financial_npv_irr_and_tvm() {
    // Net Present Value (NPV): rate = 10%, CFs = [-1000, 300, 400, 500] -> -19.124434
    let npv_val = eval("npv(0.1, -1000, 300, 400, 500)").unwrap().canonical;
    assert!((npv_val - (-19.124434)).abs() < 1e-3);

    // Internal Rate of Return (IRR): CFs = [-100, 60, 60] -> ~13.066%
    let irr_val = eval("irr(-100, 60, 60)").unwrap().canonical;
    assert!((irr_val - 0.13066).abs() < 1e-3);

    // Future Value (FV)
    let fv_val = eval("fv(0.05, 10, -1000, -10000)").unwrap().canonical;
    assert!((fv_val - 28866.83).abs() < 1e-1);
}

#[test]
fn test_statistical_distributions_and_inverse_cdfs() {
    // Normal CDF round-trip with inverse
    let p = eval("normcdf(1.95996)").unwrap().canonical;
    assert!((p - 0.975).abs() < 1e-4);

    let x = eval("invnorm(0.975)").unwrap().canonical;
    assert!((x - 1.95996).abs() < 1e-3);

    // Student's t distribution
    let t_inv = eval("invt(0.975, 10)").unwrap().canonical;
    assert!((t_inv - 2.2281388).abs() < 1e-4);

    // Binomial CDF
    let b_cdf = eval("binomcdf(10, 0.5, 5)").unwrap().canonical;
    assert!((b_cdf - 0.623046875).abs() < 1e-5);
}

#[test]
fn test_chained_unary_incrementers_and_factorials() {
    // Chained prefix increments: ++ ++5 = 7
    assert_eq!(eval("++ ++5").unwrap().to_display(), "7");

    // Double negation: - - 5 = 5
    assert_eq!(eval("- - 5").unwrap().to_display(), "5");

    // Factorial of 5 = 120
    assert_eq!(eval("5!").unwrap().to_display(), "120");
}

#[test]
fn test_niche_error_handling_and_boundary_conditions() {
    // Empty input
    assert!(matches!(eval(""), Err(AbacusError::UnexpectedEnd)));

    // Unclosed parenthesis
    assert!(matches!(eval("(5 + 3"), Err(AbacusError::UnclosedParen)));

    // Invalid inverse norm probability (> 1)
    assert!(matches!(
        eval("invnorm(1.5)"),
        Err(AbacusError::IncompatibleFunctionArguments)
    ));

    // Division by zero gives infinity
    assert_eq!(eval("5 / 0").unwrap().canonical, f64::INFINITY);

    // Negative factorial error
    assert!(matches!(
        eval("factorial(-3)"),
        Err(AbacusError::IncompatibleFunctionArguments)
    ));
}
