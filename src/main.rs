use crate::{
    registry::metric_units::register_metric_units,
    units::{unit::Unit, value::Value},
};

mod registry;
mod units;

use std::{
    collections::HashMap,
    sync::{Arc, OnceLock},
};

static METRIC_UNITS: OnceLock<HashMap<String, Arc<Unit>>> = OnceLock::new();

pub fn metric_units() -> &'static HashMap<String, Arc<Unit>> {
    METRIC_UNITS.get_or_init(register_metric_units)
}

fn main() {
    let v1 = Value::new(5.0, Arc::clone(metric_units().get("m").unwrap()));
    let v2 = Value::new(2.0, Arc::clone(metric_units().get("m").unwrap()));

    let speed = (v1 * v2).unwrap();

    println!("(5m*2m)/100m");
    println!("{}", speed.to_display());
}
