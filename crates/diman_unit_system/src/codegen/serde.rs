use proc_macro2::TokenStream;
use quote::quote;

use super::storage_types::{FloatType, VectorType};

use super::join;

use super::Codegen;

impl Codegen {
    pub fn gen_serde_impl(&self) -> TokenStream {
        join([
            self.serde_helpers_impl(),
            self.serde_floats_impl(),
            self.serde_vectors_impl(),
        ])
    }

    fn serde_helpers_impl(&self) -> TokenStream {
        let dimension_type = &self.defs.dimension_type;
        let quantity_type = &self.defs.quantity_type;
        let all_units_storage = self.runtime_unit_storage(self.defs.units.iter());

        quote! {
            use core::marker::PhantomData;
            use std::str::SplitWhitespace;

            use serde::de::{self};

            #[derive(Default)]
            struct QuantityVisitor<S, const D: #dimension_type>(PhantomData<S>);

            fn get_quantity_if_dimensions_match<S, const D: #dimension_type, E: de::Error>(
                context: &str,
                numerical_value: S,
                dimension: #dimension_type,
            ) -> Result<#quantity_type<S, D>, E> {
                if dimension == D {
                    Ok(#quantity_type::<S, D>(numerical_value))
                } else {
                    Err(E::custom(format!(
                        "mismatch in dimensions: needed: {:?} given: {:?} in string: {}",
                        D, dimension, context
                    )))
                }
            }

            fn read_unit_str<E: de::Error>(split: SplitWhitespace) -> Result<(#dimension_type, f64), E> {
                let mut total_dimension = #dimension_type::none();
                let mut total_factor = 1.0;
                for unit in split {
                    let (dimension, factor) = read_single_unit_str(unit)?;
                    total_dimension = total_dimension.add(dimension.clone());
                    total_factor *= factor;
                }
                Ok((total_dimension, total_factor))
            }

            fn read_single_unit_str<E>(unit_str: &str) -> Result<(#dimension_type, f64), E>
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
                #all_units_storage
                let unit = units
                    .get_unit_by_symbol(unit)
                    .ok_or_else(|| E::custom(format!("unknown unit: {}", &unit)))?;
                Ok((
                    unit.dimension.clone().mul(exponent),
                    Exponent::float_pow(Magnitude::from_f64(unit.magnitude), Exponent::from_int(exponent)).into_f64(),
                ))
            }
        }
    }

    fn serde_floats_impl(&self) -> TokenStream {
        self.float_types()
            .iter()
            .map(|float_type| self.serde_float_impl(float_type))
            .collect()
    }

    fn serde_float_impl(&self, float_type: &FloatType) -> TokenStream {
        let dimension_type = &self.defs.dimension_type;
        let quantity_type = &self.defs.quantity_type;
        let float_type = &float_type.name;
        quote! {
            impl<'de, const D: #dimension_type> serde::Deserialize<'de> for #quantity_type<#float_type, D> {
                fn deserialize<DE>(deserializer: DE) -> Result<#quantity_type<#float_type, D>, DE::Error>
                where
                    DE: serde::Deserializer<'de>,
                {
                    deserializer.deserialize_string(QuantityVisitor::<#float_type, D>::default())
                }
            }

            impl<'de, const D: #dimension_type> serde::de::Visitor<'de> for QuantityVisitor<#float_type, D> {
                type Value = #quantity_type<#float_type, D>;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("a numerical value followed by a series of powers of units")
                }

                fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
                where
                    E: de::Error,
                {
                    if D == #dimension_type::none() {
                        Ok(#quantity_type::<#float_type, D>(value as #float_type))
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
                    if D == #dimension_type::none() {
                        Ok(#quantity_type::<#float_type, D>(value as #float_type))
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
                    if D == #dimension_type::none() {
                        Ok(#quantity_type::<#float_type, D>(value as #float_type))
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
                    let numerical_value = numerical_value_str.parse::<#float_type>().map_err(|_| {
                        E::custom(format!(
                            "unable to parse numerical value {}",
                            &numerical_value_str
                        ))
                    })?;
                    let (total_dimension, total_factor) = read_unit_str(split)?;
                    get_quantity_if_dimensions_match::<#float_type, D, E>(
                        value,
                        (numerical_value * (total_factor as #float_type)),
                        total_dimension,
                    )
                }
            }

            impl<const D: Dimension> serde::Serialize for #quantity_type<#float_type, D> {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    serializer.serialize_str(&format!("{:?}", self))
                }
            }
        }
    }

    fn serde_vectors_impl(&self) -> TokenStream {
        self.vector_types()
            .iter()
            .map(|vector_type| self.serde_vector_impl(vector_type))
            .collect()
    }

    fn serde_vector_impl(&self, vector_type: &VectorType) -> TokenStream {
        let float_type = &vector_type.float_type.name;
        let num_dims = vector_type.num_dims;
        let vector_type = &vector_type.name;
        let dimension_type = &self.defs.dimension_type;
        let quantity_type = &self.defs.quantity_type;
        quote! {
            impl<'de, const D: #dimension_type> serde::Deserialize<'de> for #quantity_type<#vector_type, D> {
                fn deserialize<DE>(deserializer: DE) -> Result<#quantity_type<#vector_type, D>, DE::Error>
                where
                    DE: serde::Deserializer<'de>,
                {
                    deserializer.deserialize_string(QuantityVisitor::<#vector_type, D>::default())
                }
            }

            impl<'de, const D: #dimension_type> serde::de::Visitor<'de> for QuantityVisitor<#vector_type, D> {
                type Value = #quantity_type<#vector_type, D>;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    let num_expected = match #num_dims {
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
                    if vector_components.len() != #num_dims {
                        return Err(E::custom(format!("found {} substrings in brackets, expected {}", vector_components.len(), #num_dims)))?;
                    }
                    let mut array = [0.0; #num_dims];
                    for dim in 0..#num_dims {
                        let string = vector_components[dim];
                        array[dim] = string
                            .parse::<#float_type>()
                                .map_err(|e| E::custom(format!("While parsing component {}: {}, '{}'", dim, e, string)))?;

                    }
                    let vector = <#vector_type>::from_array(array);
                    let (total_dimension, total_factor) = read_unit_str(unit_part.split_whitespace())?;
                    get_quantity_if_dimensions_match::<#vector_type, D, E>(
                        value,
                        (total_factor as #float_type) * vector,
                        total_dimension,
                    )
                }
            }

            impl<const D: Dimension> serde::Serialize for #quantity_type<#vector_type, D> {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    // yaml syntax struggles with comma delimited [] entries because
                    // they look like lists, so do this ugly stuff
                    let vec_str = format!("{:?}", self);
                    serializer.serialize_str(&vec_str.replace("[", "(").replace("]", ")").replace(",", ""))
                }
            }
        }
    }
}
