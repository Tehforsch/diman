#![feature(proc_macro_diagnostic)]

mod codegen;
mod derive_dimension;
mod dimension_math;
mod expression;
mod parse;
mod resolve;
mod storage_types;
mod types;
mod verify;

use derive_dimension::dimension_impl;
use syn::*;
use verify::Verify;

// To properly do this doctest, I probably need to document this in diman itself so I can use the
// dimension. Also, a surrounding module around dimension/unit_system is needed to make the doctest work
// due to the way it is compiled.
/// Create a system of units.
/// Usage:
/// ```rust ignore
/// #![allow(incomplete_features)]
/// #![feature(generic_const_exprs, adt_const_params)]
/// use diman::unit_system;
/// use diman::dimension;
///
/// #[dimension]
/// pub struct Dimension {
///     pub length: i32,
///     pub time: i32,
/// }
///
/// unit_system!(
///     Quantity,
///     Dimension,
///     [
///         def Length = { length: 1 },
///         def Time = { time: 1 },
///         def Velocity = Length / Time,
///         unit (meters, "m") = Length,
///         unit (kilometers, "km") = 1000.0 * meters,
///         unit (seconds, "s") = 1.0 * Time,
///         unit hours = 3600 * seconds,
///         unit meters_per_second = meters / seconds,
///         unit kilometers_per_hour = kilometers / hours,
///         constant MY_FAVORITE_VELOCITY = 1000 * meters_per_second,
///     ]
/// );
/// ```
#[proc_macro]
pub fn unit_system(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let defs = parse_macro_input!(item as parse::types::Defs);
    let defs: Result<types::UnresolvedDefs> = defs.verify();
    match defs {
        Err(err) => err.to_compile_error().into(),
        Ok(defs) => {
            let resolved = defs.resolve();
            resolved.code_gen().into()
        }
    }
}

/// Derives all required methods for a dimension type.
/// Only works on structs on which every field is `i32`.
/// Also adds derives of `PartialEq`, `Eq`, `Clone` and `Debug`.
#[proc_macro_attribute]
pub fn dimension(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    dimension_impl(input)
}
