use crate::{
    expression::MulDiv,
    types::{BaseDimensionEntry, BaseDimensions},
};

#[derive(Clone)]
pub struct DimensionsAndFactor {
    pub dimensions: BaseDimensions,
    pub factor: f64,
}

impl BaseDimensions {
    pub fn none() -> Self {
        Self { fields: vec![] }
    }
}

impl std::ops::Mul for DimensionsAndFactor {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut fields = self.dimensions.fields;
        for f2 in rhs.dimensions.fields {
            let same_field = fields.iter_mut().find(|f1| f1.ident == f2.ident);
            if let Some(same_field) = same_field {
                same_field.value += f2.value;
            } else {
                fields.push(f2);
            }
        }
        Self {
            dimensions: BaseDimensions { fields },
            factor: self.factor * rhs.factor,
        }
    }
}

impl std::ops::Div for DimensionsAndFactor {
    type Output = Self;

    // :D
    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inv()
    }
}

impl MulDiv for DimensionsAndFactor {
    fn powi(self, pow: i32) -> Self {
        Self {
            factor: self.factor.powi(pow),
            dimensions: BaseDimensions {
                fields: self
                    .dimensions
                    .fields
                    .into_iter()
                    .map(|entry| BaseDimensionEntry {
                        ident: entry.ident,
                        value: entry.value * pow,
                    })
                    .collect(),
            },
        }
    }
}

impl DimensionsAndFactor {
    fn inv(mut self) -> Self {
        for field in self.dimensions.fields.iter_mut() {
            field.value = -field.value;
        }
        self.factor = 1.0 / self.factor;
        self
    }
}
