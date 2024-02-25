//! Diman is a library for zero-cost compile time unit checking.
//!
//! ```
//! # #![feature(generic_const_exprs)]
//! use diman::si::dimensions::{Length, Time, Velocity};
//! use diman::si::units::{seconds, meters, kilometers, hours};
//!
//! fn get_velocity(x: Length<f64>, t: Time<f64>) -> Velocity<f64> {
//!     x / t
//! }
//!
//! let v1 = get_velocity(36.0 * kilometers, 1.0 * hours);
//! let v2 = get_velocity(10.0 * meters, 1.0 * seconds);
//!
//! assert_eq!(v1, v2);
//! ```
//!
//! Let's try to add quantities with incompatible dimensions:
//! ```compile_fail
//! # use diman::si::units::{seconds, meters};
//! let time = 1.0 * seconds;
//! let length = 10.0 * meters;
//! let sum = length + time;
//! ```
//! This results in a compiler error:
//! ```text
//! let sum = length + time;
//!                    ^^^^
//! = note: expected struct `Quantity<_, Dimension { length: 1, time: 0, mass: 0, temperature: 0, current: 0, amount_of_substance: 0, luminous_intensity: 0 }>`
//!         found struct `Quantity<_, Dimension { length: 0, time: 1, mass: 0, temperature: 0, current: 0, amount_of_substance: 0, luminous_intensity: 0 }>`
//! ```
//!
//!
//! # Disclaimer
//! Diman is implemented using Rust's const generics feature. While `min_const_generics` has been stabilized since Rust 1.51, Diman uses more complex generic expressions and therefore requires the two currently unstable features `generic_const_exprs` and `adt_const_params`.
//!
//! Moreover, Diman is in its early stages of development and APIs will change.
//!
//! If you cannot use unstable Rust for your project or require a stable library, consider using [`uom`](https://crates.io/crates/uom) or [`dimensioned`](https://crates.io/crates/dimensioned), both of which do not require any experimental features and are much more mature libraries in general.
//!
//! # Features
//! * Invalid operations between physical quantities (adding length and time, for example) turn into compile errors.
//! * Newly created quantities are automatically converted to an underlying base representation. This means that the used types are dimensions (such as `Length`) instead of concrete units (such as `meters`) which makes for more meaningful code.
//! * Systems of dimensions and units can be user defined via the `unit_system!` macro. This gives the user complete freedom over the choice of dimensions and makes them part of the user's library, so that arbitrary new methods can be implemented on them.
//! * The `rational-dimensions` features allows the usage of quantities and units with rational exponents.
//! * `f32` and `f64` float storage types (behind the `f32` and `f64` feature gate respectively).
//! * The `std` feature is enabled by default. If disabled, Diman will be a `no_std` crate, thus suitable for use on embedded devices such as GPU device kernels.
//! * The `num-traits-libm` feature uses [libm](https://crates.io/crates/libm) to provide math functions in `no_std` environments. While one can use libm in `std`, the libm implementations are generally slower so this is unlikely to be desirable.
//! * Vector storage types via [`glam`](https://crates.io/crates/glam/) (behind the `glam-vec2`, `glam-vec3`, `glam-dvec2` and `glam-dvec3` features).
//! * Serialization and Deserialization via [`serde`](https://crates.io/crates/serde) (behind the `serde` feature gate, see the official documentation for more info).
//! * HDF5 support using [`hdf5-rs`](https://crates.io/crates/hdf5-rs/) (behind the `hdf5` feature gate).
//! * Quantities implement the `Equivalence` trait so that they can be sent via MPI using [`mpi`](https://crates.io/crates/mpi) (behind the `mpi` feature gate).
//! * Random quantities can be generated via [`rand`](https://crates.io/crates/rand) (behind the `rand` feature gate, see the official documentation for more info).
//!
//! # Design
//! Diman aims to make it as easy as possible to add compile-time unit safety to Rust code. Physical quantities are represented by the `Quantity<S, D>` struct, where `S` is the underlying storage type (`f32`, `f64`, ...) and `D` is the  dimension of the quantity. For example, in order to represent the [SI system of units](https://www.nist.gov/pml/owm/metric-si/si-units), the quantity type would be defined using the `unit_system!` macro as follows:
//! ```
//! # #![allow(incomplete_features)]
//! # #![feature(generic_const_exprs, adt_const_params)]
//! # mod surround {
//! diman::unit_system!(
//!     quantity_type Quantity;
//!     dimension_type Dimension;
//!
//!     dimension Length;
//!     dimension Time;
//!     dimension Mass;
//!     dimension Temperature;
//!     dimension Current;
//!     dimension AmountOfSubstance;
//!     dimension LuminousIntensity;
//! );
//! # }
//! ```
//! The first two statements imply that the macro should define a `Quantity` type, which is user-facing, and a `Dimension` type, which is used only internally and will surface in compiler error messages.
//! The macro will automatically implement all the required traits and methods for the `Quantity` type, such that addition and subtraction of two quantities is only allowed for quantities with the same `Dimension` type. During multiplication of two quantities, all the entries of the two dimensions are added. See below for a more comprehensive list of the implemented methods on `Quantity`.
//!
//! The `unit_system!` macro also allows defining derived dimensions and units:
//!
//! ```
//! # #![allow(incomplete_features)]
//! # #![feature(generic_const_exprs, adt_const_params)]
//! # mod surround {
//! diman::unit_system!(
//!     quantity_type Quantity;
//!     dimension_type Dimension;
//!
//!     dimension Length;
//!     dimension Time;
//!
//!     dimension Velocity = Length / Time;
//!
//!     #[prefix(kilo, milli)]
//!     #[base(Length)]
//!     #[symbol(m)]
//!     unit meters;
//!
//!     #[base(Time)]
//!     #[symbol(s)]
//!     unit seconds;
//!
//!     unit hours: Time = 3600 * seconds;
//!     unit meters_per_second: Velocity = meters / seconds;
//!     unit kilometers_per_hour: Velocity = kilometers / hours;
//!     constant MY_FAVORITE_VELOCITY = 1000 * kilometers_per_hour;
//! );
//! # }
//!
//! # use surround::dimensions::{Length, Time, Velocity};
//! # use surround::units::{meters_per_second,kilometers,hours};
//! # use surround::constants::MY_FAVORITE_VELOCITY;
//!
//! fn fast_enough(x: Length<f64>, t: Time<f64>) {
//!     let vel = x / t;
//!     if vel > 1.0 * MY_FAVORITE_VELOCITY {
//!         println!("{} m/s is definitely fast enough!", vel.value_in(meters_per_second));
//!     }
//! }
//!
//! fast_enough(100.0 * kilometers, 0.3 * hours);
//! ```
//!
//! Here, `dimension` defines Quantities, which are concrete types, `unit` defines units, which are methods on the corresponding quantities and `constant` defines constants.
//! Dimensions without a right hand side are base dimensions (such as length, time, mass, temperature, ... in the SI system of units), whereas dimensions with a right hand side are derived dimensions.
//! The same thing holds for units - every unit is either a base unit for a given base dimension (denoted by the `#[base(...)]` attribute), or derived from base units and other derived units. Base units have the special property that the internal representation of the quantity will be in terms of the base unit (for example, a stored value `1.0` for a quantity with a `Length` dimension corresponds to `meter` in the above definitions).
//! Other than this, there are no differences between base dimensions and dimensions or base units and units and they can be treated equally in user code.
//! The macro also accepts more complex expressions such as `dimension Energy = Mass (Length / Time)^2`.
//! The definitions do not have to be in any specific order.
//!
//! # The Quantity type
//! The macro will automatically implement numerical traits such as `Add`, `Sub`, `Mul`, and various other methods of the underlying storage type for `Quantity<S, ...>`.
//! `Quantity` should behave just like its underlying storage type whenever possible and allowed by the dimensions.
//! For example:
//! * Addition of `Quantity<Float, D>` and `Float` is possible if and only if `D` is dimensionless.
//! * `Quantity` implements the dimensionless methods of `S`, such as `abs` for dimensionless quantities.
//! * It implements `Deref` to `S` if and only if `D` is dimensionless.
//! * `Debug` is implemented and will print the quantity in its representation of the "closest" unit. For example `Length::meters(100.0)` would be debug printed as `0.1 km`. If printing in a specific unit is required, conversion methods are available for each unit (such as `Length::in_meters`).
//! * `.value()` provides access to the underlying storage type of a dimensionless quantity.
//! * `.value_unchecked()` provides access to the underlying storage type for all quantities if absolutely required. This is not unit-safe since the value will depend on the unit system!
//! * Similarly, new quantities can be constructed from storage types using `Quantity::new_unchecked`. This is also not unit-safe.
//!
//! Some other, more complex operations are also allowed:
//! ```
//! # use diman::si::dimensions::{Length, Volume};
//! # use diman::si::units::{meters,cubic_meters};
//! let x = 3.0f64 * meters;
//! let vol = x.cubed();
//! assert_eq!(vol, 27.0 * cubic_meters)
//! ```
//! This includes `squared`, `cubed`, `sqrt`, `cbrt` as well as `powi`.
//!
//! # Prefixes
//! Unit prefixes can automatically be generated with the `#[prefix(...)]` attribute for unit statements.
//! For example
//! ```
//! # #![allow(incomplete_features)]
//! # #![feature(generic_const_exprs, adt_const_params)]
//! # mod surround {
//! # diman_unit_system::unit_system!(
//! # quantity_type Quantity;
//! # dimension_type Dimension;
//! # dimension Length;
//! #[base(Length)]
//! #[prefix(kilo, milli)]
//! #[symbol(m)]
//! unit meters;
//! # );
//! # }
//! ```
//! will automatically generate the unit `meters` with symbol `m`, as well as `kilometers` and `millimeters` with symbols `km` and `mm` corresponding to `1e3 m` and `1e-3 m`.
//! For simplicity, the attribute `#[metric_prefixes]` is provided, which will generate all metric prefixes from `atto-` to `exa-` automatically.
//!
//! # Aliases
//! Unit aliases can automatically be generated with the `#[alias(...)]` macro. For example
//! ```
//! # #![allow(incomplete_features)]
//! # #![feature(generic_const_exprs, adt_const_params)]
//! # mod surround {
//! # diman_unit_system::unit_system!(
//! # quantity_type Quantity;
//! # dimension_type Dimension;
//! # dimension Length;
//! # #[symbol(m)]
//! # #[base(Length)]
//! #[alias(metres)]
//! unit meters;
//! # );
//! # }
//! ```
//! will automatically generate a unit `metres` that has exactly the same definition as `meters`. This works with prefixes as expected (i.e. an alias is generated for every prefixed unit).
//!
//! # Quantity products and quotients
//! Sometimes, intermediate types in computations are quantities that don't really have a nice name and are also
//! not needed too many times. Having to add a definition to the unit system for this case can be cumbersome.
//! This is why the `Product` and `Quotient` types are provided:
//! ```
//! use diman::si::dimensions::{Length, Time};
//! use diman::{Product, Quotient};
//! fn foo(l: Length<f64>, t: Time<f64>) -> Product<Length<f64>, Time<f64>> {
//!     l * t
//! }
//!
//! fn bar(l: Length<f64>, t: Time<f64>) -> Quotient<Length<f64>, Time<f64>> {
//!     l / t
//! }
//! ```
//!
//! # Rational dimensions
//! The `rational-dimensions` feature allows using quantities with rational exponents in their base dimensions, as opposed to just integer values. This allows expressing defining dimensions and units such as:
//! ```ignore
//! # mod surround {
//! # use diman_unit_system::unit_system;
//! # unit_system!(
//! # quantity_type Quantity;
//! # dimension_type Dimension;
//! # dimension Length;
//! # dimension Time;
//! # #[base(Length)]
//! # #[symbol(m)]
//! # unit meters;
//! # #[base(Time)]
//! # #[symbol(s)]
//! # unit seconds;
//! dimension Sorptivity = Length Time^(-1/2);
//! unit meters_per_sqrt_second: Sorptivity = meters / seconds^(1/2);
//! # );
//! # }
//! # use surround::dimensions::Sorptivity;
//! # use surround::units::{micrometers,milliseconds};
//! let l = 2.0 * micrometers;
//! let t = 5.0 * milliseconds;
//! let sorptivity: Sorptivity = l / t.sqrt();
//! ```
//!
//! The unit system generated with `rational-dimensions` supports a superset of features of a unit system generated without them.
//! Still, this feature should be enabled only when necessary, since the compiler errors in case of dimension mismatches will be harder to read.
//!
//! # `serde`
//! Serialization and deserialization of the units is provided via `serde` if the `serde` feature gate is enabled:
//! ```ignore
//! # use diman::si::dimensions::{Length, Velocity};
//! # use diman::si::units::{meters, meters_per_second};
//! # use serde::{Serialize, Deserialize};
//! #[derive(Serialize, Deserialize, Debug, PartialEq)]
//! struct Parameters {
//!     my_length: Length<f64>,
//!     my_vel: Velocity<f64>,
//! }
//!
//! let params: Parameters =
//!      serde_yaml::from_str("
//!         my_length: 100 m
//!         my_vel: 10 m s^-1
//!     ").unwrap();
//! assert_eq!(
//!     params,
//!     Parameters {
//!         my_length: 100.0 * meters,
//!         my_vel: 10.0 * meters_per_second,
//!     }
//! )
//! ```
//!
//! # `rand`
//! Diman allows generating random quantities via `rand` if the `rand` feature gate is enabled:
//! ```ignore
//! # use rand::Rng;
//! # use diman::si::units::{meters, kilometers};
//!
//! let mut rng = rand::thread_rng();
//! for _ in 0..100 {
//!     let start = 0.0 * meters;
//!     let end = 1.0 * kilometers;
//!     let x = rng.gen_range(start..end);
//!     assert!(Length::meters(0.0) <= x);
//!     assert!(x < Length::meters(1000.0));
//! }
//! ```

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs, adt_const_params)]
// clippy bug: https://github.com/rust-lang/rust-clippy/issues/12133
#![allow(clippy::unconditional_recursion)]

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
/// ```
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
/// ```
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
/// ```
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
/// ```
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
/// # use diman::si::dimensions::{Length, Time};
/// # use diman::si::units::{meters, seconds};
/// # use diman::Product;
/// let x: Product<Length<f64>, Time<f64>> = 20.0 * meters * seconds;
/// ```
pub type Product<Q1, Q2> = <Q1 as ::core::ops::Mul<Q2>>::Output;

/// Constructs a quotient of two quantities for one-off quantities.
/// ```
/// # #![feature(generic_const_exprs)]
/// # use diman::si::dimensions::{Length, Time};
/// # use diman::si::units::{meters, seconds};
/// # use diman::Quotient;
/// let x: Quotient<Length<f64>, Time<f64>> = 10.0 * meters / seconds;
/// ```
pub type Quotient<Q1, Q2> = <Q1 as core::ops::Div<Q2>>::Output;

pub mod internal {
    pub use diman_lib::*;
    pub mod num_traits_reexport {
        #[cfg(feature = "num-traits-libm")]
        pub use num_traits::float::Float;
        #[cfg(not(feature = "num-traits-libm"))]
        pub use num_traits::float::FloatCore;
    }
}
