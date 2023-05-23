use crate::types::Dimensions;

impl std::ops::Mul for Dimensions {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut fields = self.fields;
        for f2 in rhs.fields {
            let same_field = fields.iter_mut().find(|f1| f1.ident == f2.ident);
            if let Some(same_field) = same_field {
                same_field.value += f2.value;
            }
            else {
                fields.push(f2);
            }
        }
        Self {
            fields
        }
    }
}

impl std::ops::Div for Dimensions {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inv()
    }
}

impl Dimensions {
    fn inv(mut self) -> Self {
        for field in self.fields.iter_mut() {
            field.value = -field.value;
        }
        self
    }
}
