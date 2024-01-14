use std::collections::HashMap;

use diman_lib::{dimension_exponent::DimensionExponent, magnitude::Magnitude};
use proc_macro2::Ident;

use crate::{
    types::expression::MulDiv,
    types::{base_dimension::BaseDimension, Exponent},
};

#[derive(Clone)]
pub struct BaseDimensions {
    fields: HashMap<BaseDimension, Exponent>,
}

#[derive(Clone)]
pub struct DimensionsAndMagnitude {
    pub dimensions: BaseDimensions,
    pub magnitude: Magnitude,
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

    pub fn for_base_dimension(base_dim: BaseDimension) -> Self {
        let mut fields = HashMap::new();
        fields.insert(base_dim, Exponent::one());
        Self { fields }
    }

    pub(crate) fn fields(&self) -> impl Iterator<Item = (&Ident, &Exponent)> {
        self.fields.iter().map(|(dim, exp)| (&dim.0, exp))
    }

    pub(crate) fn keys(&self) -> impl Iterator<Item = &BaseDimension> {
        self.fields.keys()
    }

    pub(crate) fn num_fields(&self) -> usize {
        self.fields.len()
    }

    pub(crate) fn get(&self, dim: &BaseDimension) -> Option<&Exponent> {
        self.fields.get(dim)
    }
}

impl core::ops::Mul for BaseDimensions {
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

impl core::ops::Div for BaseDimensions {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inv()
    }
}

impl MulDiv for BaseDimensions {
    fn pow(self, pow: Exponent) -> Self {
        BaseDimensions {
            fields: self
                .fields
                .into_iter()
                .map(|(ident, value)| (ident, value * pow))
                .collect(),
        }
    }
}

impl DimensionsAndMagnitude {
    pub fn magnitude(magnitude: Magnitude) -> Self {
        Self {
            dimensions: BaseDimensions::none(),
            magnitude,
        }
    }

    pub(crate) fn dimensions(dimensions: BaseDimensions) -> Self {
        Self {
            dimensions,
            magnitude: Magnitude::new(1.0),
        }
    }
}

impl core::ops::Mul for DimensionsAndMagnitude {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            dimensions: self.dimensions * rhs.dimensions,
            magnitude: self.magnitude * rhs.magnitude,
        }
    }
}

impl core::ops::Div for DimensionsAndMagnitude {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            dimensions: self.dimensions / rhs.dimensions,
            magnitude: self.magnitude / rhs.magnitude,
        }
    }
}

impl MulDiv for DimensionsAndMagnitude {
    fn pow(self, pow: Exponent) -> Self {
        Self {
            magnitude: Exponent::float_pow(self.magnitude, pow),
            dimensions: self.dimensions.pow(pow),
        }
    }
}
