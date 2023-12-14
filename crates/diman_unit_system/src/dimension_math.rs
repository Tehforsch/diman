use crate::{
    expression::MulDiv,
    types::{BaseDimensionEntry, BaseDimensions},
};

impl BaseDimensions {
    pub fn none() -> Self {
        Self { fields: vec![] }
    }
}

impl std::ops::Mul for BaseDimensions {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut fields = self.fields;
        for f2 in rhs.fields {
            let same_field = fields.iter_mut().find(|f1| f1.ident == f2.ident);
            if let Some(same_field) = same_field {
                same_field.value += f2.value;
            } else {
                fields.push(f2);
            }
        }
        Self { fields }
    }
}

impl BaseDimensions {
    fn inv(mut self) -> Self {
        for field in self.fields.iter_mut() {
            field.value = -field.value;
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
    fn powi(self, pow: i32) -> Self {
        BaseDimensions {
            fields: self
                .fields
                .into_iter()
                .map(|entry| BaseDimensionEntry {
                    ident: entry.ident,
                    value: entry.value * pow,
                })
                .collect(),
        }
    }
}

#[derive(Clone)]
pub struct DimensionsAndFactor {
    pub dimensions: BaseDimensions,
    pub factor: f64,
}

impl PartialEq<BaseDimensions> for DimensionsAndFactor {
    fn eq(&self, other: &BaseDimensions) -> bool {
        self.dimensions == *other
    }
}

impl DimensionsAndFactor {
    pub fn factor(factor: f64) -> Self {
        Self {
            dimensions: BaseDimensions::none(),
            factor,
        }
    }
}

impl std::ops::Mul for DimensionsAndFactor {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            dimensions: self.dimensions * rhs.dimensions,
            factor: self.factor * rhs.factor,
        }
    }
}

impl std::ops::Div for DimensionsAndFactor {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            dimensions: self.dimensions / rhs.dimensions,
            factor: self.factor / rhs.factor,
        }
    }
}

impl MulDiv for DimensionsAndFactor {
    fn powi(self, pow: i32) -> Self {
        Self {
            factor: self.factor.powi(pow),
            dimensions: self.dimensions.powi(pow),
        }
    }
}
