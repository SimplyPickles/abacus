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

#[test]
fn test_last_thursday_at_3pm() {
    let abacus = Abacus::standard();

    let d = abacus.eval_date("last thursday at 3pm").unwrap();
    assert_eq!(d.day_of_week(), abacus::DayOfWeek::Thursday);
    assert_eq!(d.time.hour, 15);
    assert_eq!(d.time.minute, 0);

    // Verify it is in the past relative to today
    let today = Date::today();
    assert!(d.to_epoch_days() <= today.to_epoch_days());
    assert!((today.to_epoch_days() - d.to_epoch_days()) <= 7);
}

#[test]
fn test_relative_date_arithmetic_with_last_thursday() {
    let abacus = Abacus::standard();

    // Adding days to "last thursday at 3pm"
    let d_plus = abacus.eval_date("last thursday at 3pm + 2 days").unwrap();
    let d_orig = abacus.eval_date("last thursday at 3pm").unwrap();
    assert_eq!(d_plus, d_orig.add_days(2));
    assert_eq!(d_plus.time.hour, 15);

    // Subtracting hours
    let d_sub = abacus.eval_date("last thursday at 3pm - 3 hours").unwrap();
    assert_eq!(d_sub.time.hour, 12);

    // Property access
    let hour = abacus.eval_scalar("last thursday at 3pm.hour").unwrap();
    assert_eq!(hour.canonical, 15.0);

    let dow = abacus.eval_scalar("last thursday at 3pm.day_of_week").unwrap();
    assert_eq!(dow.canonical, 4.0); // Thursday = 4
}

#[test]
fn test_relative_weekday_modifiers() {
    let abacus = Abacus::standard();
    let today = Date::today();

    let d_next_mon = abacus.eval_date("next monday at 10am").unwrap();
    assert_eq!(d_next_mon.day_of_week(), abacus::DayOfWeek::Monday);
    assert_eq!(d_next_mon.time.hour, 10);
    assert!(d_next_mon.to_epoch_days() > today.to_epoch_days());

    let d_this_fri = abacus.eval_date("this friday at 4:30pm").unwrap();
    assert_eq!(d_this_fri.day_of_week(), abacus::DayOfWeek::Friday);
    assert_eq!(d_this_fri.time.hour, 16);
    assert_eq!(d_this_fri.time.minute, 30);

    let d_prev_tue = abacus.eval_date("previous tuesday at 14:00").unwrap();
    assert_eq!(d_prev_tue.day_of_week(), abacus::DayOfWeek::Tuesday);
    assert_eq!(d_prev_tue.time.hour, 14);
    assert!(d_prev_tue.to_epoch_days() <= today.to_epoch_days());
}

#[test]
fn test_standalone_time_literals() {
    let abacus = Abacus::standard();

    let d1 = abacus.eval_date("3pm").unwrap();
    assert_eq!(d1.year, Date::today().year);
    assert_eq!(d1.month, Date::today().month);
    assert_eq!(d1.day, Date::today().day);
    assert_eq!(d1.time.hour, 15);

    let d2 = abacus.eval_date("9:15 AM").unwrap();
    assert_eq!(d2.time.hour, 9);
    assert_eq!(d2.time.minute, 15);
}
