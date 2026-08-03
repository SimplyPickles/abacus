use std::ops::{Add, Mul, Sub};

pub const DIMENSION_COUNT: usize = 8;

#[derive(Debug, Clone, Copy, Default)]
pub struct Dimensions(pub [f64; DIMENSION_COUNT]);

impl PartialEq for Dimensions {
    fn eq(&self, other: &Self) -> bool {
        self.0.iter().zip(other.0.iter()).all(|(a, b)| (a - b).abs() < 1e-9)
    }
}

// Since we implement PartialEq with epsilon, we can safely implement Eq for use in existing structs (like Unit).
impl Eq for Dimensions {}

impl Dimensions {
    pub const DIMENSIONLESS: Self = Self([0.0; DIMENSION_COUNT]);

    pub const LENGTH: Self = Self([1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
    pub const MASS: Self = Self([0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
    pub const TIME: Self = Self([0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
    pub const CURRENT: Self = Self([0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0]);
    pub const TEMPERATURE: Self = Self([0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0]);
    pub const AMOUNT: Self = Self([0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0]);
    pub const LUMINOUS_INTENSITY: Self = Self([0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0]);
    pub const INFORMATION: Self = Self([0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0]);

    pub const AREA: Self = Self([2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
    pub const VOLUME: Self = Self([3.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);

    pub fn is_dimensionless(self) -> bool {
        self == Self::DIMENSIONLESS
    }
}

impl Add for Dimensions {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(std::array::from_fn(|i| self.0[i] + rhs.0[i]))
    }
}

impl Sub for Dimensions {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = [0.0; DIMENSION_COUNT];

        for (index, output) in result.iter_mut().enumerate() {
            *output = self.0[index] - rhs.0[index];
        }

        Self(result)
    }
}

impl Mul<f64> for Dimensions {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        let mut result = [0.0; DIMENSION_COUNT];

        for (index, output) in result.iter_mut().enumerate() {
            *output = self.0[index] * rhs;
        }

        Self(result)
    }
}
