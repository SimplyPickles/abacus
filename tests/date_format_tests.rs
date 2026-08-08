use abacus::{Abacus, DateFormat};

#[test]
fn test_default_date_format_is_dd_mm_yyyy() {
    let abacus = Abacus::standard();
    assert_eq!(abacus.date_format, DateFormat::DDMMYYYY);

    let d_res = abacus.eval("07-08-2026").unwrap();
    assert_eq!(d_res.to_display(), "07-08-2026");
}

#[test]
fn test_configurable_date_format() {
    let abacus_iso = Abacus::standard().with_date_format(DateFormat::YYYYMMDD);
    let d = abacus_iso.eval_date("07-08-2026").unwrap();
    assert_eq!(d.format_with_style(abacus_iso.date_format), "2026-08-07");

    let abacus_us = Abacus::standard().with_date_format(DateFormat::MMDDYYYY);
    let d2 = abacus_us.eval_date("07-08-2026").unwrap();
    assert_eq!(d2.format_with_style(abacus_us.date_format), "08-07-2026");
}

#[test]
fn test_date_to_date_time_between_conversion() {
    let abacus = Abacus::standard();

    // 07-08-2026 to 17-08-2026 yields 10 days in hours by default (240 h)
    let res_hours_default = abacus.eval("07-08-2026 to 17-08-2026").unwrap();
    assert_eq!(res_hours_default.to_display(), "240 h");

    // Chained conversion in days
    let res_days = abacus.eval("07-08-2026 to 17-08-2026 in days").unwrap();
    assert_eq!(res_days.to_display(), "10 d");

    // Chained conversion in hours
    let res_hours = abacus.eval("07-08-2026 to 08-08-2026 in hours").unwrap();
    assert_eq!(res_hours.to_display(), "24 h");
}
