#![allow(incomplete_features)]
#![feature(generic_const_exprs, adt_const_params)]
#![doc = include_str!("../README.md")]
#![cfg_attr(
    feature = "serde",
    doc = r#"
# Serde
Serialization and deserialization of the units is provided via `serde` if the `serde` feature gate is enabled
```rust
use diman::si::f64::{Length, Velocity};
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Parameters {
    my_length: Length,
    my_vel: Velocity,
}

let params: Parameters = 
     serde_yaml::from_str("
        my_length: 100 m
        my_vel: 10 m s^-1
    ").unwrap();
assert_eq!(
    params, 
    Parameters {
        my_length: Length::meters(100.0),
        my_vel: Velocity::meters_per_second(10.0),
    }
)
```
"#
)]
#![cfg_attr(
    feature = "rand",
    doc = r#"
# Rand
Diman allows generating random quantities via `rand`:
```rust
use rand::Rng;

use diman::si::f64::Length;

let mut rng = rand::thread_rng();
for _ in 0..100 {
    let x = rng.gen_range(Length::meters(0.0)..Length::kilometers(1.0));
    assert!(Length::meters(0.0) <= x);
    assert!(x < Length::meters(1000.0));
}
```
"#
)]

mod debug_storage_type;
mod type_aliases;

#[cfg(feature = "si")]
/// Defines the dimensions and units for the SI system.
pub mod si;

pub use debug_storage_type::DebugStorageType;
pub use type_aliases::Product;
pub use type_aliases::QProduct;
pub use type_aliases::Quotient;

// The surrounding module around the unit_system calls is needed to make the doctest work
// due to the way it is compiled.
/// Create a quantity type for a given system of dimensions and units.
/// The macro requires a list of statements separated with semicolons.
/// There are five different types of statements, depending on the leading keyword:
/// 1. `quantity_type NAME`: The name of the quantity type that will be defined.
/// 2. `dimension_type NAME`: The name of the dimension type that will be defined. Appears in error messages but is otherwise hidden from the user.
/// 3. `dimension`: Define a new dimension. If no expression is given (as in `dimension Length;`), this will define a new base dimension. If an expression is given, it will define a type alias for a derived dimension (as in `dimension Velocity = Length / Time`).
/// 4. `constant`: Define a new constant. Example: `constant ELECTRON_CHARGE = 1.602176634e-19 volts`.
/// 5. `unit`: Define a new unit. If no expression is given and the `#[base(...)]` attribute is set, it will be the base unit for the given dimension. Example:
/// ```rust
/// # #![allow(incomplete_features)]
/// # #![feature(generic_const_exprs, adt_const_params)]
/// # mod surround {
/// # use diman_unit_system::unit_system;
/// # unit_system!(
/// # quantity_type Quantity;
/// # dimension_type Dimension;
/// # dimension Length;
/// #[base(Length)]
/// unit meter
/// # );
/// # }
/// ```
/// Derived units can be defined via expressions, such as
/// ```rust
/// # #![allow(incomplete_features)]
/// # #![feature(generic_const_exprs, adt_const_params)]
/// # mod surround {
/// # use diman_unit_system::unit_system;
/// # unit_system!(
/// # quantity_type Quantity;
/// # dimension_type Dimension;
/// # dimension Length;
/// # #[base(Length)]
/// # unit meter;
/// unit foot = 0.3048 * meter;
/// # );
/// # }
/// ```
/// Unit statements may optionally be annotated with their resulting dimension to prevent bugs:
/// ```rust
/// # #![allow(incomplete_features)]
/// # #![feature(generic_const_exprs, adt_const_params)]
/// # mod surround {
/// # use diman_unit_system::unit_system;
/// # unit_system!(
/// # quantity_type Quantity;
/// # dimension_type Dimension;
/// # dimension Length;
/// # #[base(Length)]
/// # unit meter;
/// unit foot: Length = 0.3048 * meter;
/// # );
/// # }
/// ```
/// Unit prefixes can be generated automatically using the `#[prefix(...)]` attribute for the unit statement.
/// All metric prefixes (from atto- to exa-) can be generated automatically using the `#[metric_prefixes]` attribute for the unit statement.
/// Aliases of the unit can be defined using the `#[alias(...)]` attribute.
/// The symbol of the unit can be defined using the `#[symbol(...)]` attribute.
///
/// Example usage:
/// ```rust
/// # #![allow(incomplete_features)]
/// # #![feature(generic_const_exprs, adt_const_params)]
/// # mod surround {
/// # use diman_unit_system::unit_system;
/// unit_system!(
///     quantity_type Quantity;
///     dimension_type Dimension;
///
///     dimension Length;
///     dimension Time;
///
///     dimension Velocity = Length / Time;
///
///     #[prefix(kilo, milli)]
///     #[symbol(m)]
///     #[base(Length)]
///     unit meters;
///
///     #[base(Time)]
///     unit seconds;
///
///     unit hours: Time = 3600 * seconds;
///     unit meters_per_second: Velocity = meters / seconds;
///     unit kilometers_per_hour: Velocity = kilometers / hours;
///     constant MY_FAVORITE_VELOCITY = 1000 * kilometers_per_hour;
/// );
/// # }
/// ```
pub use diman_unit_system::unit_system;
