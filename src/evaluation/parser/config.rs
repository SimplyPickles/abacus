use crate::{AngleMode, IntervalStyle, TimeZone, WeekendDays};

/// Configuration settings controlling evaluation semantics.
#[derive(Debug, Clone)]
pub struct EvalConfig {
    pub auto_derived: bool,
    pub angle_mode: AngleMode,
    pub strict_dimensions: bool,
    pub default_interval_style: Option<IntervalStyle>,
    pub default_timezone: Option<TimeZone>,
    pub weekend: WeekendDays,
    pub max_recursion_depth: usize,
    pub implicit_multiplication: bool,
    pub number_scales: bool,
    pub currencies: bool,
    pub live_rates: bool,
}

impl Default for EvalConfig {
    fn default() -> Self {
        Self {
            auto_derived: true,
            angle_mode: AngleMode::Radians,
            strict_dimensions: false,
            default_interval_style: None,
            default_timezone: None,
            weekend: WeekendDays::SaturdaySunday,
            max_recursion_depth: 64,
            implicit_multiplication: true,
            number_scales: cfg!(feature = "number-scales"),
            currencies: cfg!(feature = "currencies"),
            live_rates: false,
        }
    }
}
