# Diman
Diman is a library for zero-cost compile time unit checking.

```rust
use diman::si::f64::{Length, Time, Velocity};

fn get_velocity(x: Length, t: Time) -> Velocity {
    x / t
}

let v1 = get_velocity(Length::kilometers(36.0), Time::hours(1.0));
let v2 = get_velocity(Length::meters(10.0), Time::seconds(1.0));

assert_eq!(v1, v2);
```

Let's try to assign add quantities with incompatible dimensions:
```rust compile_fail
use diman::si::f64::{Length, Time};

let time = Time::seconds(1.0);
let length = Length::meters(10.0);
let sum = length + time;
```
This results in a compiler error:
```text
let sum = length + time;
                   ^^^^
= note: expected struct `Quantity<_, Dimension { length: 1, time: 0, mass: 0, temperature: 0, current: 0, amount_of_substance: 0, luminous_intensity: 0 }>`
        found struct `Quantity<_, Dimension { length: 0, time: 1, mass: 0, temperature: 0, current: 0, amount_of_substance: 0, luminous_intensity: 0 }>`
```


## Disclaimer
Diman is implemented using Rust's const generics feature. While `min_const_generics` has been stabilized since Rust 1.51, Diman uses more complex generic expressions and therefore requires the two currently unstable features `generic_const_exprs` and `adt_const_params`. 

Moreover, Diman is in its early stages of development and APIs will change.

If you cannot use unstable Rust for your project or require a stable library, consider using [`uom`](https://crates.io/crates/uom) or [`dimensioned`](https://crates.io/crates/dimensioned), both of which do not require any experimental features and are much more mature libraries in general.

## Features
* Invalid operations between physical quantities (adding length and time, for example) turn into compile errors.
* Newly created quantities are automatically converted to an underlying base representation. This means that the used types are dimensions (such as `Length`) instead of concrete units (such as `meters`) which makes for more meaningful code.
* Systems of dimensions and units can be user defined via the `unit_system!` macro. This gives the user complete freedom over the choice of dimensions and makes them part of the user's library, so that arbitrary new methods can be implemented on them.
* The `rational-dimensions` features allows the usage of quantities and units with rational exponents.
* `f32` and `f64` float storage types (behind the `f32` and `f64` feature gate respectively).
* Vector storage types via [`glam`](https://crates.io/crates/glam/) (behind the `glam-vec2`, `glam-vec3`, `glam-dvec2` and `glam-dvec3` features).
* Serialization and Deserialization via [`serde`](https://crates.io/crates/serde) (behind the `serde` feature gate, see the official documentation for more info).
* HDF5 support using [`hdf5-rs`](https://crates.io/crates/hdf5-rs/) (behind the `hdf5` feature gate).
* Quantities implement the `Equivalence` trait so that they can be sent via MPI using [`mpi`](https://crates.io/crates/mpi) (behind the `mpi` feature gate).
* Random quantities can be generated via [`rand`](https://crates.io/crates/rand) (behind the `rand` feature gate, see the official documentation for more info).

## Design
Diman aims to make it as easy as possible to add compile-time unit safety to Rust code. Physical quantities are represented by the `Quantity<S, D>` struct, where `S` is the underlying storage type (`f32`, `f64`, ...) and `D` is the  dimension of the quantity. For example, in order to represent the [SI system of units](https://www.nist.gov/pml/owm/metric-si/si-units), the quantity type would be defined using the `unit_system!` macro as follows:
```rust ignore
#![allow(incomplete_features)]
#![feature(generic_const_exprs, adt_const_params)]
use diman::unit_system;

unit_system!(
    quantity_type Quantity;
    dimension_type Dimension;

    dimension Length;
    dimension Time;
    dimension Mass;
    dimension Temperature;
    dimension Current;
    dimension AmountOfSubstance;
    dimension LuminousIntensity;
);
```
The first two statements imply that the macro should define a `Quantity` type, which is user-facing, and a `Dimension` type, which is used only internally and will surface in compiler error messages.
The macro will automatically implement all the required traits and methods for the `Quantity` type, such that addition and subtraction of two quantities is only allowed for quantities with the same `Dimension` type. During multiplication of two quantities, all the entries of the two dimensions are added. See below for a more comprehensive list of the implemented methods on `Quantity`.

The `unit_system!` macro also allows defining derived dimensions and units:

```rust ignore
#![allow(incomplete_features)]
#![feature(generic_const_exprs, adt_const_params)]
use diman_unit_system::unit_system;
unit_system!(
    quantity_type Quantity;
    dimension_type Dimension;

    dimension Length;
    dimension Time;

    dimension Velocity = Length / Time;

    #[prefix(kilo, milli)]
    #[symbol(m)]
    #[base(Length)]
    unit meters;

    #[base(Time)]
    unit seconds;

    unit hours: Time = 3600 * seconds;
    unit meters_per_second: Velocity = meters / seconds;
    unit kilometers_per_hour: Velocity = kilometers / hours;
    constant MY_FAVORITE_VELOCITY = 1000 * kilometers_per_hour;
);

use f64::{Length, Time, Velocity, MY_FAVORITE_VELOCITY};

fn fast_enough(x: Length, t: Time) {
    let vel = x / t;
    if vel > MY_FAVORITE_VELOCITY {
        println!("{} m/s is definitely fast enough!", vel.in_meters_per_second());
    }
}

fast_enough(Length::kilometers(100.0), Time::hours(0.3));
```

Here, `dimension` defines Quantities, which are concrete types, `unit` defines units, which are methods on the corresponding quantities and `constant` defines constants.
Dimensions without a right hand side are base dimensions (such as length, time, mass, temperature, ... in the SI system of units), whereas dimensions with a right hand side are derived dimensions.
The same thing holds for units - every unit is either a base unit for a given base dimension (denoted by the `#[base(...)]` attribute), or derived from base units and other derived units. Base units have the special property that the internal representation of the quantity will be in terms of the base unit (for example, a stored value `1.0` for a quantity with a `Length` dimension corresponds to `meter` in the above definitions).
Other than this, there are no differences between base dimensions and dimensions or base units and units and they can be treated equally in user code.
The macro also accepts more complex expressions such as `dimension Energy = Mass (Length / Time)^2`.
The definitions do not have to be in any specific order.

## The Quantity type
The macro will automatically implement numerical traits such as `Add`, `Sub`, `Mul`, and various other methods of the underlying storage type for `Quantity<S, ...>`.
`Quantity` should behave just like its underlying storage type whenever possible and allowed by the dimensions. 
For example:
* Addition of `Quantity<Float, D>` and `Float` is possible if and only if `D` is dimensionless.
* `Quantity` implements the dimensionless methods of `S`, such as `abs` for dimensionless quantities.
* It implements `Deref` to `S` if and only if `D` is dimensionless.
* `Debug` is implemented and will print the quantity in its representation of the "closest" unit. For example `Length::meters(100.0)` would be debug printed as `0.1 km`. If printing in a specific unit is required, conversion methods are available for each unit (such as `Length::in_meters`).
* `.value()` provides access to the underlying storage type of a dimensionless quantity.
* `.value_unchecked()` provides access to the underlying storage type for all quantities if absolutely required. This is not unit-safe since the value will depend on the unit system!
* Similarly, new quantities can be constructed from storage types using `Quantity::new_unchecked`. This is also not unit-safe.

Some other, more complex operations are also allowed:
```
use diman::si::f64::{Length, Volume};
let x = Length::meters(3.0);
let vol = x.cubed();
assert_eq!(vol, Volume::cubic_meters(27.0))
```
This includes `squared`, `cubed`, `sqrt`, `cbrt` as well as `powi`.

## Prefixes
Unit prefixes can automatically be generated with the `#[prefix(...)]` attribute for unit statements.
For example
```rust ignore
#[prefix(kilo, milli)]
#[symbol(m)]
unit meters;
```
will automatically generate the unit `meters` with symbol `m`, as well as `kilometers` and `millimeters` with symbols `km` and `mm` corresponding to `1e3 m` and `1e-3 m`.
For simplicity, the attribute `#[metric_prefixes]` is provided, which will generate all metric prefixes from `atto-` to `exa-` automatically.

## Aliases
Unit aliases can automatically be generated with the `#[alias(...)]` macro. For example
```rust ignore
#[alias(metres)]
unit meters;
```
will automatically generate a unit `metres` that has exactly the same definition as `meters`. This works with prefixes as expected (i.e. an alias is generated for every prefixed unit).

## Quantity products and quotients
Sometimes, intermediate types in computations are quantities that don't really have a nice name and are also
not needed too many times. Having to add a definition to the unit system for this case can be cumbersome.
This is why the `Product` and `Quotient` types are provided:
```rust
use diman::si::f64::{Length, Time, Velocity, Area, Volume};
use diman::{Product, Quotient};
fn foo(l: Length, t: Time, vol: Volume) -> Product<(Length, Time, Volume)> {
    l * t * vol
}

fn bar(l: Length, t: Time) -> Quotient<Length, Time> {
    l / t
}
```

## Rational dimensions
The `rational-dimensions` feature allows using quantities with rational exponents in their base dimensions, as opposed to just integer values. This allows expressing writing expressions such as
```rust ignore
let x = Length::meters(10.0);
let foo = x.sqrt();
```
Without `rational-dimensions`, this would not be allowed, since the dimension of foo is L^(1/2).
The unit system generated with `rational-dimensions` supports a superset of features of a unit system generated without them.
Still, this feature should be enabled only when necessary, since the compiler errors in case of dimension mismatches will be harder to read.

## `serde`
Serialization and deserialization of the units is provided via `serde` if the `serde` feature gate is enabled:
```rust ignore
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

## `rand`
Diman allows generating random quantities via `rand` if the `rand` feature gate is enabled:
```rust ignore
use rand::Rng;

use diman::si::f64::Length;

let mut rng = rand::thread_rng();
for _ in 0..100 {
    let x = rng.gen_range(Length::meters(0.0)..Length::kilometers(1.0));
    assert!(Length::meters(0.0) <= x);
    assert!(x < Length::meters(1000.0));
}
```
