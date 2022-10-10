mod float_quantity;
mod vector_quantity;

#[macro_export]
macro_rules! impl_serde {
    ($quantity: ident, $dimension: ident, $dimensionless_const: ident) => {
        $crate::impl_serde_helpers!($quantity, $dimension, $dimensionless_const);
        $crate::impl_serde_float!($quantity, $dimension, $dimensionless_const, f32);
        $crate::impl_serde_float!($quantity, $dimension, $dimensionless_const, f64);
        $crate::impl_serde_vector!($quantity, $dimension, $dimensionless_const, DVec2, f64, 2);
        $crate::impl_serde_vector!($quantity, $dimension, $dimensionless_const, DVec3, f64, 3);
    };
}

#[macro_export]
macro_rules! impl_serde_helpers {
    ($quantity: ident, $dimension: ident, $dimensionless_const: ident) => {
        use std::marker::PhantomData;
        use std::str::SplitWhitespace;

        use serde::de::{self};

        #[derive(Default)]
        struct QuantityVisitor<S, const D: $dimension>(PhantomData<S>);

        fn get_quantity_if_dimensions_match<S, const D: $dimension, E: de::Error>(
            context: &str,
            numerical_value: S,
            dimension: $dimension,
        ) -> Result<$quantity<S, D>, E> {
            if dimension == D {
                Ok($quantity::<S, D>(numerical_value))
            } else {
                Err(E::custom(format!(
                    "mismatch in dimensions: needed: {:?} given: {:?} in string: {}",
                    D, dimension, context
                )))
            }
        }

        fn read_unit_str<E: de::Error>(split: SplitWhitespace) -> Result<($dimension, f64), E> {
            let mut total_dimension = $dimensionless_const;
            let mut total_factor = 1.0;
            for unit in split {
                let (dimension, factor) = read_single_unit_str(unit)?;
                total_dimension = total_dimension.dimension_mul(dimension.clone());
                total_factor *= factor;
            }
            Ok((total_dimension, total_factor))
        }

        fn read_single_unit_str<E>(unit_str: &str) -> Result<($dimension, f64), E>
        where
            E: de::Error,
        {
            let (unit, exponent) = if unit_str.contains('^') {
                let split: Vec<_> = unit_str.split('^').collect();
                if split.len() != 2 {
                    return Err(E::custom(format!("invalid unit string: {}", unit_str)));
                }
                (
                    split[0],
                    split[1].parse::<i32>().map_err(|_| {
                        E::custom(format!("unable to parse unit exponent: {}", split[1]))
                    })?,
                )
            } else {
                (unit_str, 1)
            };
            let (dimension, _, factor) = UNIT_NAMES
                .iter()
                .find(|(_, known_unit_name, _)| &unit == known_unit_name)
                .ok_or_else(|| E::custom(format!("unknown unit: {}", &unit)))?;
            Ok((
                dimension.clone().dimension_powi(exponent),
                factor.powi(exponent),
            ))
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::si::Dimensionless;
    use crate::si::Force;
    use crate::si::Length;
    use crate::tests::assert_is_close;

    #[test]
    fn deserialize_basic_units() {
        let q: Length = serde_yaml::from_str("1.0 m").unwrap();
        assert_is_close(q, Length::meters(1.0));
        let q: Length = serde_yaml::from_str("2.0 m").unwrap();
        assert_is_close(q, Length::meters(2.0));
        let q: Length = serde_yaml::from_str("2.0e8 m").unwrap();
        assert_is_close(q, Length::meters(2.0e8));
        let q: Length = serde_yaml::from_str("5.0 km").unwrap();
        assert_is_close(q, Length::meters(5000.0));
    }

    #[test]
    fn deserialize_dimensionless_quantities() {
        let q: Dimensionless = serde_yaml::from_str("5.0").unwrap();
        assert_is_close(q, Dimensionless::dimensionless(5.0));
    }

    #[test]
    #[should_panic]
    fn do_not_deserialize_dimensionless_quantities_with_unit_str() {
        let q: Dimensionless = serde_yaml::from_str("5.0 m").unwrap();
        assert_is_close(q, Dimensionless::dimensionless(5.0));
    }

    #[test]
    #[should_panic]
    fn do_not_allow_unit_mismatch() {
        let _q: Dimensionless = serde_yaml::from_str("5.0 km m").unwrap();
    }

    #[test]
    fn deserialize_unit_exponents() {
        let q: Dimensionless = serde_yaml::from_str("5.0 km m^-1").unwrap();
        assert_is_close(q, Dimensionless::dimensionless(5000.0));
        let q: Force = serde_yaml::from_str("5.0 kg m^1 s^-2").unwrap();
        assert_is_close(q, Force::newtons(5.0));
        let q: Force = serde_yaml::from_str("5.0e-3 kg km^2 m^-1 s^-2").unwrap();
        assert_is_close(q, Force::newtons(5000.0));
    }
}
