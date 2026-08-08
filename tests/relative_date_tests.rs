use abacus::{Abacus, Date};

#[test]
fn test_relative_date_keywords() {
    let abacus = Abacus::standard();

    let d_today = abacus.eval_date("today").unwrap();
    let expected_today = Date::today();
    assert_eq!(d_today.year, expected_today.year);
    assert_eq!(d_today.month, expected_today.month);
    assert_eq!(d_today.day, expected_today.day);

    let d_tomorrow = abacus.eval_date("tomorrow").unwrap();
    let expected_tomorrow = Date::tomorrow();
    assert_eq!(d_tomorrow.day, expected_tomorrow.day);

    let d_yesterday = abacus.eval_date("yesterday").unwrap();
    let expected_yesterday = Date::yesterday();
    assert_eq!(d_yesterday.day, expected_yesterday.day);
}

#[test]
fn test_relative_date_with_at_time() {
    let abacus = Abacus::standard();

    let d1 = abacus.eval_date("today at 12:00").unwrap();
    assert_eq!(d1.time.hour, 12);
    assert_eq!(d1.time.minute, 0);

    let d2 = abacus.eval_date("tomorrow at 3:30 PM").unwrap();
    assert_eq!(d2.time.hour, 15);
    assert_eq!(d2.time.minute, 30);
}

#[test]
fn test_relative_time_interval_today_at_12_to_1() {
    let abacus = Abacus::standard();

    // today at 12:00 to 1:00 in hours -> 1 h (12:00 PM to 1:00 PM)
    let res = abacus.eval("today at 12:00 to 1:00 in hours").unwrap();
    assert_eq!(res.to_display(), "1 h");

    // today at 11:30 AM to 2:15 PM in hours -> 2.75 h
    let res2 = abacus
        .eval("today at 11:30 AM to 2:15 PM in hours")
        .unwrap();
    assert_eq!(res2.to_display(), "2.75 h");
}

#[test]
fn test_relative_time_interval_defaults_to_hours() {
    let abacus = Abacus::standard();

    // today at 12:00 to 3:00 -> 3 h by default!
    let res = abacus.eval("today at 12:00 to 3:00").unwrap();
    assert_eq!(res.to_display(), "3 h");

    // Overridden with explicit unit
    let res_min = abacus.eval("today at 12:00 to 3:00 in minutes").unwrap();
    assert_eq!(res_min.to_display(), "180 min");
}
