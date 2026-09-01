use crate::{
    evaluation::parser::pratt::Parser,
    units::{eval_result::EvalResult, unit::Unit, unit::UnitExpr, value::Value},
    AbacusError, Token,
};
use std::sync::Arc;

impl<'a> Parser<'a> {
    pub(crate) fn hour_unit(&self) -> Arc<Unit> {
        self.unit_registry
            .unit("h")
            .or_else(|_| self.unit_registry.unit("hour"))
            .unwrap_or_else(|_| {
                Arc::new(Unit {
                    scalar: 3600.0,
                    offset: 0.0,
                    dimensions: crate::units::dimensions::Dimensions::TIME,
                    display: UnitExpr::single("h"),
                })
            })
    }

    pub(crate) fn parse_conversion(&mut self, lhs: EvalResult) -> Result<EvalResult, AbacusError> {
        self.advance();
        self.has_explicit_conversion = true;

        if let Some(Token::Unit(u)) = self.peek() {
            let sym = *u;
            if (sym == "%" || sym == "percent" || sym == "pct")
                && self.peek_next() == Some(&Token::BinaryOp("of"))
            {
                self.advance();
                self.advance();
                let base_result = self.parse_expr(1)?;
                let base_val = base_result.into_scalar()?;
                let lhs_val = lhs.into_scalar()?;
                let pct_unit = self.unit_registry.unit("%")?;
                let ratio = lhs_val.canonical / base_val.canonical;
                return Ok(EvalResult::Scalar(Value {
                    canonical: ratio,
                    unit: pct_unit,
                }));
            }
        }

        if let EvalResult::Date(ref d1) = lhs {
            match self.peek() {
                Some(Token::Date(d2)) => {
                    let mut d2 = d2.clone();
                    self.advance();

                    // Infer 12-hour clock rollover if d2.time < d1.time on same date (e.g. 12:00 to 1:00 -> 12:00 to 13:00)
                    if d2.year == d1.year
                        && d2.month == d1.month
                        && d2.day == d1.day
                        && d2.time.hour < d1.time.hour
                        && d2.time.hour < 12
                    {
                        d2.time.hour += 12;
                    }

                    let unit_h = self.hour_unit();
                    let diff_val = (&d2 - d1).convert_to(unit_h)?;
                    return Ok(EvalResult::Scalar(diff_val));
                }
                Some(Token::Unit(u)) => {
                    let sym = *u;
                    if let Ok(tz) = crate::units::date::TimeZone::parse(sym) {
                        self.advance();
                        return Ok(EvalResult::Date(d1.to_timezone(&tz)));
                    }
                }
                _ => {}
            }
            let rhs_result = self.parse_expr(1)?;
            return match rhs_result {
                EvalResult::Date(ref d2) => {
                    let unit_h = self.hour_unit();
                    let diff_val = (d2 - d1).convert_to(unit_h)?;
                    Ok(EvalResult::Scalar(diff_val))
                }
                other => {
                    let target_str = other.unit().display.render();
                    let tz = crate::units::date::TimeZone::parse(&target_str)?;
                    Ok(EvalResult::Date(d1.to_timezone(&tz)))
                }
            };
        }

        // The RHS of a conversion must be a scalar unit expression
        let target_result = self.parse_expr(1)?;
        let target_unit = target_result.unit().clone();
        lhs.convert_to(target_unit)
    }
}
