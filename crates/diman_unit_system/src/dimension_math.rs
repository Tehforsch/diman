use std::collections::HashMap;

use proc_macro2::Ident;

use crate::{expression::MulDiv, types::BaseDimensionExponent};

#[derive(Clone)]
pub struct BaseDimensions {
    pub fields: HashMap<Ident, BaseDimensionExponent>,
}

impl PartialEq for BaseDimensions {
    fn eq(&self, other: &Self) -> bool {
        self.fields.iter().all(|(dimension, value)| {
            if let Some(corresponding_value) = other.fields.get(dimension) {
                value == corresponding_value
            } else {
                false
            }
        })
    }
}
impl BaseDimensions {
    pub fn none() -> Self {
        Self {
            fields: HashMap::default(),
        }
    }
}

impl std::ops::Mul for BaseDimensions {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn mul(self, rhs: Self) -> Self::Output {
        let mut fields = self.fields;
        for (name_rhs, val_rhs) in rhs.fields {
            let same_field = fields.get_mut(&name_rhs);
            if let Some(val) = same_field {
                *val += val_rhs;
            } else {
                fields.insert(name_rhs, val_rhs);
            }
        }
        Self { fields }
    }
}

impl BaseDimensions {
    fn inv(mut self) -> Self {
        for (_, value) in self.fields.iter_mut() {
            *value = -*value;
        }
        self
    }
}

impl std::ops::Div for BaseDimensions {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inv()
    }
}

impl MulDiv for BaseDimensions {
    fn pow(self, pow: BaseDimensionExponent) -> Self {
        BaseDimensions {
            fields: self
                .fields
                .into_iter()
                .map(|(ident, value)| (ident, value * pow))
                .collect(),
        }
    }
}

#[derive(Clone)]
pub struct DimensionsAndMagnitude {
    pub dimensions: BaseDimensions,
    pub magnitude: f64,
}

impl DimensionsAndMagnitude {
    pub fn magnitude(magnitude: f64) -> Self {
        Self {
            dimensions: BaseDimensions::none(),
            magnitude,
        }
    }

    pub(crate) fn dimensions(dimensions: BaseDimensions) -> Self {
        Self {
            dimensions,
            magnitude: 1.0,
        }
    }
}

impl std::ops::Mul for DimensionsAndMagnitude {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            dimensions: self.dimensions * rhs.dimensions,
            magnitude: self.magnitude * rhs.magnitude,
        }
    }
}

impl std::ops::Div for DimensionsAndMagnitude {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            dimensions: self.dimensions / rhs.dimensions,
            magnitude: self.magnitude / rhs.magnitude,
        }
    }
}

impl MulDiv for DimensionsAndMagnitude {
    fn pow(self, pow: BaseDimensionExponent) -> Self {
        Self {
            magnitude: BaseDimensionExponent::pow(self.magnitude, pow),
            dimensions: self.dimensions.pow(pow),
        }
    }
}
