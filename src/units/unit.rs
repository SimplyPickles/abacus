use std::ops::Deref;

use crate::{global_units, units::dimensions::Dimensions};

#[derive(Debug)]
pub struct Unit {
    pub dimensions: Dimensions,
    pub scalar: f64,
    pub offset: f64,
    pub display: UnitExpr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnitExpr {
    pub numerator: Vec<String>,
    pub denominator: Vec<String>,
}

impl UnitExpr {
    pub fn dimensionless() -> Self {
        Self {
            numerator: Vec::new(),
            denominator: Vec::new(),
        }
    }

    pub fn single(unit: impl Into<String>) -> Self {
        Self {
            numerator: vec![unit.into()],
            denominator: Vec::new(),
        }
    }

    pub fn multiply(&self, rhs: &Self) -> Self {
        let mut numerator = self.numerator.clone();
        numerator.extend(rhs.numerator.iter().cloned());

        let mut denominator = self.denominator.clone();
        denominator.extend(rhs.denominator.iter().cloned());

        Self {
            numerator,
            denominator,
        }
    }

    pub fn divide(&self, rhs: &Self) -> Self {
        let mut numerator = self.numerator.clone();
        numerator.extend(rhs.denominator.iter().cloned());

        let mut denominator = self.denominator.clone();
        denominator.extend(rhs.numerator.iter().cloned());

        Self {
            numerator,
            denominator,
        }
    }

    pub fn simplified(mut self) -> Self {
        let mut numerator_index = 0;

        while numerator_index < self.numerator.len() {
            if let Some(denominator_index) = self
                .denominator
                .iter()
                .position(|unit| unit == &self.numerator[numerator_index])
            {
                self.numerator.remove(numerator_index);
                self.denominator.remove(denominator_index);
            } else {
                numerator_index += 1;
            }
        }

        self
    }

    pub fn render(&self) -> String {
        match (self.numerator.is_empty(), self.denominator.is_empty()) {
            (true, true) => String::new(),
            (false, true) => self.render_numerator(),
            (true, false) => format!("1/{}", self.render_denominator()),
            (false, false) => format!("{}/{}", self.render_numerator(), self.render_denominator()),
        }
    }

    pub fn render_numerator(&self) -> String {
        render_units(&self.numerator)
    }

    pub fn render_denominator(&self) -> String {
        render_units(&self.denominator)
    }
}

fn render_units(units: &[String]) -> String {
    use std::collections::HashMap;

    let mut counts = HashMap::new();
    let mut order = Vec::new();

    for unit in units {
        if !counts.contains_key(unit) {
            order.push(unit);
        }
        *counts.entry(unit).or_insert(0) += 1;
    }

    order
        .into_iter()
        .map(|unit| {
            let count = counts[unit];
            if count == 1 {
                unit.clone()
            } else {
                format!("{unit}^{count}")
            }
        })
        .collect::<Vec<_>>()
        .join("*")
}

impl Unit {
    pub fn simplify_display(&mut self) {
        self.display = self.display.clone().simplified();

        if self
            .display
            .numerator
            .iter()
            .all(|unit| unit == "dimensionless")
        {
            self.display = UnitExpr::dimensionless();
        }

        for num in self.display.numerator.iter() {
            for (b, den) in self.display.denominator.iter_mut().enumerate() {
                if let Some(un1) = global_units().get(num) {
                    if let Some(un2) = global_units().get(den) {
                        if un1.dimensions == un2.dimensions {
                            self.scalar = self.scalar * (un2.scalar / un1.scalar);
                            self.display.denominator.remove(b);
                            break;
                        }
                    }
                }
            }
        }
    }

    pub fn is_compatible_with(&self, other: &Self) -> bool {
        self.dimensions == other.dimensions
    }

    pub fn is_affine(&self) -> bool {
        self.offset != 0.0
    }

    pub fn dimensionless() -> Self {
        Self {
            dimensions: Dimensions::DIMENSIONLESS,
            scalar: 1.0,
            offset: 0.0,
            display: UnitExpr::dimensionless(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_dimensionless_unit_expr() {
        assert_eq!(UnitExpr::dimensionless().render(), "");
    }

    #[test]
    fn renders_repeated_units_as_exponents() {
        let area = UnitExpr::single("m").multiply(&UnitExpr::single("m"));
        let volume = area.multiply(&UnitExpr::single("m"));

        assert_eq!(area.render(), "m^2");
        assert_eq!(volume.render(), "m^3");
    }

    #[test]
    fn renders_denominator_only_expressions() {
        let inverse_seconds = UnitExpr::dimensionless().divide(&UnitExpr::single("s"));

        assert_eq!(inverse_seconds.render(), "1/s");
    }
}
