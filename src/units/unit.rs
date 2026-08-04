use crate::units::dimensions::Dimensions;

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
    let mut parts = Vec::new();
    let mut visited = Vec::new();

    for unit in units {
        if !visited.contains(&unit) {
            visited.push(unit);
            let count = units.iter().filter(|u| u == &unit).count();
            if count == 1 {
                parts.push(unit.clone());
            } else {
                parts.push(format!("{unit}^{count}"));
            }
        }
    }

    parts.join("*")
}

use std::sync::Arc;

impl Unit {
    pub fn simplify_display_with(&self, lookup: impl Fn(&str) -> Option<Arc<Unit>>) -> Unit {
        let un = self;
        let mut scalar = un.scalar;
        let mut display = un.display.clone().simplified();

        if self
            .display
            .numerator
            .iter()
            .all(|unit| unit == "dimensionless")
        {
            display = UnitExpr::dimensionless();
        }

        let mut num_idx = 0;
        while num_idx < display.numerator.len() {
            let num_sym = &display.numerator[num_idx];
            if let Some(un1) = lookup(num_sym) {
                let mut matched = false;
                for den_idx in 0..display.denominator.len() {
                    let den_sym = &display.denominator[den_idx];
                    if let Some(un2) = lookup(den_sym) {
                        if un1.dimensions == un2.dimensions {
                            scalar *= un2.scalar / un1.scalar;
                            display.numerator.remove(num_idx);
                            display.denominator.remove(den_idx);
                            matched = true;
                            break;
                        }
                    }
                }
                if !matched {
                    num_idx += 1;
                }
            } else {
                num_idx += 1;
            }
        }

        Unit {
            dimensions: self.dimensions,
            scalar,
            offset: self.offset,
            display,
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

    pub fn is_dimensionless(&self) -> bool {
        self.dimensions == Dimensions::DIMENSIONLESS
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

    // #[test]
    // fn simplifies_compatible_cross_unit_expressions() {
    //     let mut unit = Unit {
    //         scalar: 1000.0,
    //         offset: 0.0,
    //         dimensions: Dimensions::DIMENSIONLESS,
    //         display: UnitExpr::single("km").divide(&UnitExpr::single("m")),
    //     };
    //     unit.simplify_display();

    //     assert_eq!(unit.display.render(), "");
    //     assert_eq!(unit.scalar, 1.0);
    // }
}
