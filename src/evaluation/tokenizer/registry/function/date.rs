use crate::{
    AbacusError, Date, EvalResult, Value,
    evaluation::tokenizer::registry::function::operators::FunctionOp,
};

fn date_fn(args: &[Value]) -> Result<EvalResult, AbacusError> {
    if args.len() == 3 {
        let year = args[0].canonical as i32;
        let month = args[1].canonical as u32;
        let day = args[2].canonical as u32;
        let d = Date::new(year, month, day);
        if !d.is_valid() {
            return Err(AbacusError::InvalidDate(
                "invalid calendar date".to_string(),
            ));
        }
        Ok(EvalResult::Date(d))
    } else if args.len() == 6 {
        let year = args[0].canonical as i32;
        let month = args[1].canonical as u32;
        let day = args[2].canonical as u32;
        let hour = args[3].canonical as u32;
        let minute = args[4].canonical as u32;
        let second = args[5].canonical as u32;
        let d = Date::new_with_hms(year, month, day, hour, minute, second);
        if !d.is_valid() {
            return Err(AbacusError::InvalidDate(
                "invalid date/time values".to_string(),
            ));
        }
        Ok(EvalResult::Date(d))
    } else {
        Err(AbacusError::IncompatibleFunctionArguments)
    }
}

fn date_prop_helper(args: &[EvalResult], prop: &str) -> Result<EvalResult, AbacusError> {
    if args.len() != 1 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }
    match &args[0] {
        EvalResult::Date(d) => {
            if let Some(val) = d.get_property(prop) {
                Ok(EvalResult::Scalar(Value::dimensionless(val)))
            } else {
                Err(AbacusError::IncompatibleFunctionArguments)
            }
        }
        _ => Err(AbacusError::IncompatibleFunctionArguments),
    }
}

fn year_fn(args: &[EvalResult]) -> Result<EvalResult, AbacusError> {
    date_prop_helper(args, "year")
}
fn month_fn(args: &[EvalResult]) -> Result<EvalResult, AbacusError> {
    date_prop_helper(args, "month")
}
fn day_fn(args: &[EvalResult]) -> Result<EvalResult, AbacusError> {
    date_prop_helper(args, "day")
}
fn hour_fn(args: &[EvalResult]) -> Result<EvalResult, AbacusError> {
    date_prop_helper(args, "hour")
}
fn minute_fn(args: &[EvalResult]) -> Result<EvalResult, AbacusError> {
    date_prop_helper(args, "minute")
}
fn second_fn(args: &[EvalResult]) -> Result<EvalResult, AbacusError> {
    date_prop_helper(args, "second")
}
fn millisecond_fn(args: &[EvalResult]) -> Result<EvalResult, AbacusError> {
    date_prop_helper(args, "millisecond")
}
fn day_of_week_fn(args: &[EvalResult]) -> Result<EvalResult, AbacusError> {
    date_prop_helper(args, "day_of_week")
}
fn day_of_year_fn(args: &[EvalResult]) -> Result<EvalResult, AbacusError> {
    date_prop_helper(args, "day_of_year")
}
fn is_weekend_fn(args: &[EvalResult]) -> Result<EvalResult, AbacusError> {
    date_prop_helper(args, "is_weekend")
}
fn is_workday_fn(args: &[EvalResult]) -> Result<EvalResult, AbacusError> {
    date_prop_helper(args, "is_workday")
}

fn workdays_fn(args: &[EvalResult]) -> Result<EvalResult, AbacusError> {
    if args.len() != 2 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }
    match (&args[0], &args[1]) {
        (EvalResult::Date(d1), EvalResult::Date(d2)) => {
            use crate::units::{
                dimensions::Dimensions,
                unit::{Unit, UnitExpr},
            };
            let bdays = d1.business_days_between(d2) as f64;
            let unit = std::sync::Arc::new(Unit {
                scalar: 86400.0,
                offset: 0.0,
                dimensions: Dimensions::TIME,
                display: UnitExpr::single("workdays"),
            });
            Ok(EvalResult::Scalar(Value::new(bdays, unit)))
        }
        _ => Err(AbacusError::IncompatibleFunctionArguments),
    }
}

fn format_date_fn(args: &[EvalResult]) -> Result<EvalResult, AbacusError> {
    if args.is_empty() || args.len() > 2 {
        return Err(AbacusError::IncompatibleFunctionArguments);
    }
    match &args[0] {
        EvalResult::Date(d) => {
            let style = if args.len() == 2 {
                let fmt_str = match &args[1] {
                    EvalResult::Scalar(v) => v.to_units_display().to_ascii_uppercase(),
                    other => other.to_display().to_ascii_uppercase(),
                };
                if fmt_str.contains("YYYY") && fmt_str.starts_with('Y') {
                    crate::units::date::DateFormat::YYYYMMDD
                } else if fmt_str.starts_with('M') {
                    crate::units::date::DateFormat::MMDDYYYY
                } else {
                    crate::units::date::DateFormat::DDMMYYYY
                }
            } else {
                crate::units::date::DateFormat::DDMMYYYY
            };
            let mut formatted_d = d.clone();
            formatted_d.format = style;
            Ok(EvalResult::Date(formatted_d))
        }
        _ => Err(AbacusError::IncompatibleFunctionArguments),
    }
}

pub fn register_date_functions() -> Vec<FunctionOp> {
    vec![
        FunctionOp::eval_result("date", 3, 6, date_fn),
        FunctionOp::general("year", 1, 1, year_fn),
        FunctionOp::general("month", 1, 1, month_fn),
        FunctionOp::general("day", 1, 1, day_fn),
        FunctionOp::general("hour", 1, 1, hour_fn),
        FunctionOp::general("minute", 1, 1, minute_fn),
        FunctionOp::general("second", 1, 1, second_fn),
        FunctionOp::general("millisecond", 1, 1, millisecond_fn),
        FunctionOp::general("day_of_week", 1, 1, day_of_week_fn),
        FunctionOp::general("weekday", 1, 1, day_of_week_fn),
        FunctionOp::general("day_of_year", 1, 1, day_of_year_fn),
        FunctionOp::general("is_weekend", 1, 1, is_weekend_fn),
        FunctionOp::general("is_workday", 1, 1, is_workday_fn),
        FunctionOp::general("is_business_day", 1, 1, is_workday_fn),
        FunctionOp::general("workdays", 2, 2, workdays_fn),
        FunctionOp::general("business_days", 2, 2, workdays_fn),
        FunctionOp::general("format_date", 1, 2, format_date_fn),
    ]
}
