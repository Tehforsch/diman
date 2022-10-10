#[macro_export]
macro_rules! impl_serde_vector {
    ($quantity: ident, $dimension: ident, $dimensionless_const: ident, $vector_type: ident, $float_type: ident, $num_dims: literal) => {
        impl<'de, const D: $dimension> serde::Deserialize<'de> for $quantity<$vector_type, D> {
            fn deserialize<DE>(deserializer: DE) -> Result<$quantity<$vector_type, D>, DE::Error>
            where
                DE: serde::Deserializer<'de>,
            {
                deserializer.deserialize_string(QuantityVisitor::<$vector_type, D>::default())
            }
        }

        impl<'de, const D: $dimension> serde::de::Visitor<'de> for QuantityVisitor<$vector_type, D> {
            type Value = $quantity<$vector_type, D>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                let num_expected = match $num_dims {
                    2 => "two",
                    3 => "three",
                    _ => unimplemented!(),
                };
                formatter.write_str(&format!("{} numerical values surrounded by () followed by a series of powers of units, e.g. (1.0 2.0) m s^-2", num_expected))
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let value = value.trim();
                let bracket_end = value
                    .find(')')
                    .ok_or_else(|| E::custom("No closing bracket in vector string"))?;
                let (vector_part, unit_part) = value.split_at(bracket_end + 1);
                let bracket_begin = vector_part
                    .find('(')
                    .ok_or_else(|| E::custom("No opening bracket in vector string"))?;
                let vector_part = vector_part[bracket_begin + 1..vector_part.len() - 1].to_string();
                let vector_components = &vector_part.split_whitespace().collect::<Vec<_>>();
                if vector_components.len() != $num_dims {
                    return Err(E::custom(format!("found {} substrings in brackets, expected {}", vector_components.len(), $num_dims)))?;
                }
                let mut array = [0.0; $num_dims];
                for dim in 0..$num_dims {
                    let string = vector_components[dim];
                    array[dim] = string
                        .parse::<$float_type>()
                            .map_err(|e| E::custom(format!("While parsing component {}: {}, '{}'", dim, e, string)))?;

                }
                let vector = $vector_type::from_array(array);
                let (total_dimension, total_factor) = read_unit_str(unit_part.split_whitespace())?;
                get_quantity_if_dimensions_match::<$vector_type, D, E>(
                    value,
                    (total_factor as $float_type) * vector,
                    total_dimension,
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::si::DVec2Length;
    use crate::si::DVec3Length;
    use crate::si::Length;
    use crate::tests::assert_is_close;

    #[test]
    fn deserialize_vector_2() {
        let q: DVec2Length = serde_yaml::from_str("(5.0 3.0) km").unwrap();
        assert_is_close(q.x(), Length::kilometers(5.0));
        assert_is_close(q.y(), Length::kilometers(3.0));
    }

    #[test]
    fn deserialize_vector_3() {
        let q: DVec3Length = serde_yaml::from_str("(5.0 3.0 7.0) km").unwrap();
        assert_is_close(q.x(), Length::kilometers(5.0));
        assert_is_close(q.y(), Length::kilometers(3.0));
        assert_is_close(q.z(), Length::kilometers(7.0));
    }

    #[test]
    #[should_panic]
    fn deserialize_vector_2_fails_with_fewer_than_2_components() {
        let _: DVec2Length = serde_yaml::from_str("(5.0) km").unwrap();
    }

    #[test]
    #[should_panic]
    fn deserialize_vector_2_fails_with_more_than_2_components() {
        let _: DVec2Length = serde_yaml::from_str("(5.0 3.0 7.0) km").unwrap();
    }

    #[test]
    #[should_panic]
    fn deserialize_vector_3_fails_with_fewer_than_3_components() {
        let _: DVec3Length = serde_yaml::from_str("(5.0 4.0) km").unwrap();
    }

    #[test]
    #[should_panic]
    fn deserialize_vector_3_fails_with_more_than_3_components() {
        let _: DVec3Length = serde_yaml::from_str("(5.0 3.0 7.0 9.0) km").unwrap();
    }
}
