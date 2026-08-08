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

fn date_property_dummy(_args: &[Value]) -> Result<EvalResult, AbacusError> {
    Err(AbacusError::IncompatibleFunctionArguments)
}

pub fn register_date_functions() -> Vec<FunctionOp> {
    vec![
        FunctionOp::eval_result("date", 3, 6, date_fn),
        FunctionOp::eval_result("year", 1, 1, date_property_dummy),
        FunctionOp::eval_result("month", 1, 1, date_property_dummy),
        FunctionOp::eval_result("day", 1, 1, date_property_dummy),
        FunctionOp::eval_result("hour", 1, 1, date_property_dummy),
        FunctionOp::eval_result("minute", 1, 1, date_property_dummy),
        FunctionOp::eval_result("second", 1, 1, date_property_dummy),
        FunctionOp::eval_result("millisecond", 1, 1, date_property_dummy),
        FunctionOp::eval_result("day_of_week", 1, 1, date_property_dummy),
        FunctionOp::eval_result("weekday", 1, 1, date_property_dummy),
        FunctionOp::eval_result("day_of_year", 1, 1, date_property_dummy),
        FunctionOp::eval_result("is_weekend", 1, 1, date_property_dummy),
        FunctionOp::eval_result("is_workday", 1, 1, date_property_dummy),
        FunctionOp::eval_result("is_business_day", 1, 1, date_property_dummy),
        FunctionOp::eval_result("workdays", 2, 2, date_property_dummy),
        FunctionOp::eval_result("business_days", 2, 2, date_property_dummy),
    ]
}
