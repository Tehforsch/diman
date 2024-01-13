#![cfg_attr(not(feature = "std"), no_std)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs, adt_const_params)]
#![doc = include_str!("../README.md")]

// This ensures we don't have to differentiate between
// imports via `crate::` and `diman::` in the proc macro.
extern crate self as diman;

#[cfg(all(
    feature = "rational-dimensions",
    not(any(feature = "std", feature = "num-traits-libm"))
))]
compile_error!(
    "The \"rational-dimensions\" feature requires either \"std\" or \"num-traits-libm\""
);

#[cfg(feature = "si")]
/// Defines the dimensions and units for the SI system.
pub mod si;

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
/// #[symbol(m)]
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
/// # #[symbol(m)]
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
/// # #[symbol(m)]
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
///     #[base(Length)]
///     #[prefix(kilo, milli)]
///     #[symbol(m)]
///     unit meters;
///
///     #[base(Time)]
///     #[symbol(s)]
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

/// Constructs a product of quantities for one-off quantities.
/// ```
/// # #![feature(generic_const_exprs)]
/// # use diman::si::f64::{Length, Time};
/// # use diman::Product;
/// let x: Product<Length, Time> = Length::meters(10.0) * Time::seconds(2.0);
/// ```
pub type Product<Q1, Q2> = <Q1 as ::core::ops::Mul<Q2>>::Output;

/// Constructs a quotient of two quantities for one-off quantities.
/// ```
/// # #![feature(generic_const_exprs)]
/// # use diman::si::f64::{Length, Time};
/// # use diman::Quotient;
/// let x: Quotient<Length, Time> = Length::meters(10.0) / Time::seconds(2.0);
/// ```
pub type Quotient<Q1, Q2> = <Q1 as core::ops::Div<Q2>>::Output;

pub use diman_lib::ratio::Ratio;
pub use diman_lib::runtime_unit_storage;
