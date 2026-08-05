use abacus::{Abacus, AbacusError, EvalResult};

fn eval(expr: &str) -> Result<EvalResult, AbacusError> {
    Abacus::standard().eval(expr)
}

fn eval_scalar(expr: &str) -> Result<abacus::Value, AbacusError> {
    Abacus::standard().eval_scalar(expr)
}

// ── TInterval (TI-84 Option 8) ──

#[test]
fn test_ti84_t_interval_sample_data() {
    // Tests TInterval with sample data: TInterval(10 m, 12 m, 11 m, 14 m)
    let result = eval("TInterval(10 m, 12 m, 11 m, 14 m)").unwrap();
    let display = result.to_display();
    assert!(
        display.starts_with("[") && display.ends_with("]"),
        "Got display: {}",
        display
    );

    if let EvalResult::Interval(interval) = result {
        let lo = (interval.lo.canonical - interval.lo.unit.offset) / interval.lo.unit.scalar;
        let hi = (interval.hi.canonical - interval.hi.unit.offset) / interval.hi.unit.scalar;
        assert!(
            (lo - 9.0326).abs() < 0.05,
            "lo should be ~9.03 m, got {}",
            lo
        );
        assert!(
            (hi - 14.4674).abs() < 0.05,
            "hi should be ~14.47 m, got {}",
            hi
        );
    } else {
        panic!("Expected interval result");
    }
}

#[test]
fn test_ti84_t_interval_summary_stats() {
    // Tests TInterval with summary statistics: TInterval(100 m, 15 m, 25)
    let result = eval("TInterval(100 m, 15 m, 25)").unwrap();
    let display = result.to_display();
    assert!(
        display.contains("m"),
        "Expected meters in display: {}",
        display
    );

    if let EvalResult::Interval(interval) = result {
        let lo = (interval.lo.canonical - interval.lo.unit.offset) / interval.lo.unit.scalar;
        let hi = (interval.hi.canonical - interval.hi.unit.offset) / interval.hi.unit.scalar;
        assert!(
            (lo - 93.8083).abs() < 0.05,
            "lo should be ~93.81, got {}",
            lo
        );
        assert!(
            (hi - 106.1917).abs() < 0.05,
            "hi should be ~106.19, got {}",
            hi
        );
    } else {
        panic!("Expected interval result");
    }
}

// ── ZInterval (TI-84 Option 7) ──

#[test]
fn test_ti84_z_interval_summary_stats() {
    // Tests ZInterval with summary statistics: ZInterval(100 m, 15 m, 100)
    let result = eval("ZInterval(100 m, 15 m, 100)").unwrap();
    if let EvalResult::Interval(interval) = result {
        let lo = (interval.lo.canonical - interval.lo.unit.offset) / interval.lo.unit.scalar;
        let hi = (interval.hi.canonical - interval.hi.unit.offset) / interval.hi.unit.scalar;
        assert!((lo - 97.06).abs() < 0.05, "lo should be ~97.06, got {}", lo);
        assert!(
            (hi - 102.94).abs() < 0.05,
            "hi should be ~102.94, got {}",
            hi
        );
    } else {
        panic!("Expected interval result");
    }
}

// ── 1-PropZInt (TI-84 Option A) ──

#[test]
fn test_ti84_1_prop_z_int() {
    // Tests 1-PropZInt(45, 100) and 1_PropZInt(45, 100)
    let result1 = eval("1-PropZInt(45, 100)").unwrap();
    let result2 = eval("1_PropZInt(45, 100)").unwrap();
    assert_eq!(result1.to_display(), result2.to_display());

    if let EvalResult::Interval(interval) = result1 {
        let lo = interval.lo.canonical;
        let hi = interval.hi.canonical;
        assert!((lo - 0.355).abs() < 0.02, "lo should be ~0.355, got {}", lo);
        assert!((hi - 0.548).abs() < 0.02, "hi should be ~0.548, got {}", hi);
    } else {
        panic!("Expected interval result");
    }
}

// ── 2-SampTInt (TI-84 Option 0) ──

#[test]
fn test_ti84_2_samp_t_int() {
    // Tests 2-SampTInt(100 m, 15 m, 25, 90 m, 10 m, 30)
    let result = eval("2-SampTInt(100 m, 15 m, 25, 90 m, 10 m, 30)").unwrap();
    let display = result.to_display();
    assert!(
        display.starts_with("[") && display.contains("m"),
        "Got display: {}",
        display
    );

    if let EvalResult::Interval(interval) = result {
        let lo = (interval.lo.canonical - interval.lo.unit.offset) / interval.lo.unit.scalar;
        let hi = (interval.hi.canonical - interval.hi.unit.offset) / interval.hi.unit.scalar;
        // mean diff = 10 m, Welch df ≈ 41.3, SE ≈ 3.51 m, t*(95%) ≈ 2.02, MoE ≈ 7.09 m -> [2.91 m, 17.09 m]
        assert!((lo - 2.91).abs() < 0.5, "lo should be ~2.91 m, got {}", lo);
        assert!(
            (hi - 17.09).abs() < 0.5,
            "hi should be ~17.09 m, got {}",
            hi
        );
    } else {
        panic!("Expected interval result");
    }
}

// ── 2-SampZInt (TI-84 Option 9) ──

#[test]
fn test_ti84_2_samp_z_int() {
    // Tests 2-SampZInt(100 m, 15 m, 50, 90 m, 10 m, 50)
    let result = eval("2-SampZInt(100 m, 15 m, 50, 90 m, 10 m, 50)").unwrap();
    if let EvalResult::Interval(interval) = result {
        let lo = (interval.lo.canonical - interval.lo.unit.offset) / interval.lo.unit.scalar;
        let hi = (interval.hi.canonical - interval.hi.unit.offset) / interval.hi.unit.scalar;
        // mean diff = 10 m, SE = sqrt(2.25 + 2.0) ≈ 2.55 m, z*(95%) ≈ 1.96, MoE ≈ 5.0 m -> [5.0 m, 15.0 m]
        assert!((lo - 5.0).abs() < 0.5, "lo should be ~5.0 m, got {}", lo);
        assert!((hi - 15.0).abs() < 0.5, "hi should be ~15.0 m, got {}", hi);
    } else {
        panic!("Expected interval result");
    }
}

// ── 2-PropZInt (TI-84 Option B) ──

#[test]
fn test_ti84_2_prop_z_int() {
    // Tests 2-PropZInt(45, 100, 30, 100)
    let result = eval("2-PropZInt(45, 100, 30, 100)").unwrap();
    if let EvalResult::Interval(interval) = result {
        let lo = interval.lo.canonical;
        let hi = interval.hi.canonical;
        // p1 = 0.45, p2 = 0.30, diff = 0.15, SE ≈ 0.0676, z*(95%) ≈ 1.96, MoE ≈ 0.132 -> [0.018, 0.282]
        assert!((lo - 0.018).abs() < 0.02, "lo should be ~0.018, got {}", lo);
        assert!((hi - 0.282).abs() < 0.02, "hi should be ~0.282, got {}", hi);
    } else {
        panic!("Expected interval result");
    }
}

// ── Margin of Error (moe, tmoe, zmoe) ──

#[test]
fn test_moe_scalar_margin_of_error() {
    let moe_val = eval_scalar("moe(10 m, 12 m, 11 m, 14 m)").unwrap();
    assert_eq!(moe_val.unit.display.render(), "m");
    assert!((moe_val.canonical - 2.7174).abs() < 0.05);

    let tmoe_val = eval_scalar("tmoe(15 m, 25)").unwrap();
    assert_eq!(tmoe_val.unit.display.render(), "m");
    assert!((tmoe_val.canonical - 6.1917).abs() < 0.05);

    let zmoe_val = eval_scalar("zmoe(15 m, 100)").unwrap();
    assert_eq!(zmoe_val.unit.display.render(), "m");
    assert!((zmoe_val.canonical - 2.94).abs() < 0.05);
}

// ── Interval arithmetic with TI-84 Confidence Intervals ──

#[test]
fn test_ti84_interval_with_arithmetic() {
    // TInterval(10 m, 12 m, 11 m, 14 m) * 2
    let result = eval("TInterval(10 m, 12 m, 11 m, 14 m) * 2").unwrap();
    let display = result.to_display();
    assert!(
        display.starts_with("[") && display.contains("m"),
        "Got display: {}",
        display
    );
}
