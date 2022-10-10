#[macro_export]
macro_rules! impl_serde_float {
    ($quantity: ident, $dimension: ident, $dimensionless_const: ident, $float_type: ident) => {
        impl<'de, const D: $dimension> serde::Deserialize<'de> for $quantity<$float_type, D> {
            fn deserialize<DE>(deserializer: DE) -> Result<$quantity<$float_type, D>, DE::Error>
            where
                DE: serde::Deserializer<'de>,
            {
                deserializer.deserialize_string(QuantityVisitor::<$float_type, D>::default())
            }
        }

        impl<'de, const D: $dimension> serde::de::Visitor<'de> for QuantityVisitor<$float_type, D> {
            type Value = $quantity<$float_type, D>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a numerical value followed by a series of powers of units")
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if D == NONE {
                    Ok($quantity::<$float_type, D>(value as $float_type))
                } else {
                    Err(E::custom(format!(
                        "dimensionless numerical value given for non-dimensionless quantity: {}",
                        value
                    )))
                }
            }
            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if D == NONE {
                    Ok($quantity::<$float_type, D>(value as $float_type))
                } else {
                    Err(E::custom(format!(
                        "dimensionless numerical value given for non-dimensionless quantity: {}",
                        value
                    )))
                }
            }

            fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if D == NONE {
                    Ok($quantity::<$float_type, D>(value as $float_type))
                } else {
                    Err(E::custom(format!(
                        "dimensionless numerical value given for non-dimensionless quantity: {}",
                        value
                    )))
                }
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let value = value.trim();
                let mut split = value.split_whitespace();
                let numerical_value_str = split
                    .next()
                    .ok_or_else(|| E::custom("unable to parse empty string"))?;
                let numerical_value = numerical_value_str.parse::<$float_type>().map_err(|_| {
                    E::custom(format!(
                        "unable to parse numerical value {}",
                        &numerical_value_str
                    ))
                })?;
                let (total_dimension, total_factor) = read_unit_str(split)?;
                get_quantity_if_dimensions_match::<$float_type, D, E>(
                    value,
                    (numerical_value * (total_factor as $float_type)),
                    total_dimension,
                )
            }
        }
    };
}
