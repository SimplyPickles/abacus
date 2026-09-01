use abacus::Abacus;

#[test]
fn test_auto_derived_units_default_enabled() {
    let calc = Abacus::standard();
    assert!(calc.auto_derived_units);

    // 10 N * 5 m -> 50 J
    let res = calc.eval("10 N * 5 m").unwrap();
    assert_eq!(res.to_display(), "50 J");

    // 100 W * 5 s -> 500 J
    let res = calc.eval("100 W * 5 s").unwrap();
    assert_eq!(res.to_display(), "500 J");

    // 12 V * 2 A -> 24 W
    let res = calc.eval("12 V * 2 A").unwrap();
    assert_eq!(res.to_display(), "24 W");
}

#[test]
fn test_auto_derived_units_disabled() {
    let calc = Abacus::standard().with_auto_derived_units(false);
    assert!(!calc.auto_derived_units);

    // 10 N * 5 m -> 50 N*m (not reduced to J)
    let res = calc.eval("10 N * 5 m").unwrap();
    assert_eq!(res.to_display(), "50 N*m");

    // 100 W * 5 s -> 500 W*s (not reduced to J)
    let res = calc.eval("100 W * 5 s").unwrap();
    assert_eq!(res.to_display(), "500 W*s");

    // Explicit conversion overrides the disabled auto-reduction
    let explicit = calc.eval("(10 N * 5 m) to J").unwrap();
    assert_eq!(explicit.to_display(), "50 J");
}

#[test]
fn test_fixed_significant_figures() {
    let calc = Abacus::standard().with_significant_figures(3);
    assert_eq!(calc.significant_figures, Some(3));

    // 12.3456 m -> 12.3 m
    let res = calc.eval("12.3456 m").unwrap();
    assert_eq!(res.to_display(), "12.3 m");

    // 12345.6 m -> 12300 m
    let res = calc.eval("12345.6 m").unwrap();
    assert_eq!(res.to_display(), "12300 m");

    // 0.004567 s with 2 sig figs
    let calc2 = Abacus::standard().with_significant_figures(2);
    let res2 = calc2.eval("0.004567 s").unwrap();
    assert_eq!(res2.to_display(), "0.0046 s");

    // Formatted trailing zeros via format_result
    let res3 = calc.eval("1.2 m").unwrap();
    assert_eq!(calc.format_result(&res3), "1.20 m");

    // Degenerate/Interval rounding
    let int_res = calc.eval("[1.2345 m, 2.3456 m]").unwrap();
    assert_eq!(int_res.to_display(), "[1.23 m, 2.35 m]");
}

#[test]
fn test_follow_input_significant_figures() {
    let calc = Abacus::standard().with_follow_significant_figures(true);
    assert!(calc.follow_significant_figures);

    // 12.3 (3 sig figs) * 4.567 (4 sig figs) -> minimum is 3 sig figs
    // 12.3 * 4.567 = 56.1741 -> rounded to 3 sig figs = 56.2
    let res = calc.eval("12.3 * 4.567").unwrap();
    assert_eq!(res.to_display(), "56.2");

    // 2.50 m (3 sig figs) * 4.0 m (2 sig figs) -> min is 2 sig figs = 10 m^2
    let res = calc.eval("2.50 m * 4.0 m").unwrap();
    assert_eq!(res.to_display(), "10 m^2");
}

#[test]
fn test_in_place_configuration_mutators() {
    let mut calc = Abacus::standard();

    calc.set_auto_derive(false);
    assert!(!calc.auto_derived_units);

    calc.set_sig_figs(Some(4));
    assert_eq!(calc.significant_figures, Some(4));

    calc.set_follow_sig_figs(true);
    assert!(calc.follow_significant_figures);

    let res = calc.eval("10 N * 5 m").unwrap();
    assert_eq!(res.to_display(), "50 N*m");
}
