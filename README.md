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

Let's try to assign add quantities with incompatible units:
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
* Invalid operations between physical quantities turn into compile errors.
* Newly created quantities are automatically converted to an underlying base representation. This means that the used types are quantities (such as `Length`) instead of concrete units (such as `meters`) which makes for more meaningful code.
* Systems of units and quantities can be user defined via the `unit_system!` macro. This gives the user complete freedom over the choice of quantities and makes them part of the user's library, so that arbitrary new methods can be implemented on them.
* `f32` and `f64` float storage types (behind the `f32` and `f64` feature gate respectively).
* Vector storage types via [`glam`](https://crates.io/crates/glam/) (behind the `glam-vec2`, `glam-vec3`, `glam-dvec2` and `glam-dvec3` features).
* Serialization and Deserialization via [`serde`](https://crates.io/crates/serde) (behind the `serde` feature gate, see the official documentation for more info).
* HDF5 support using [`hdf5-rs`](https://crates.io/crates/hdf5-rs/) (behind the `hdf5` feature gate).
* Quantities implement the `Equivalence` trait so that they can be sent via MPI using [`mpi`](https://crates.io/crates/mpi) (behind the `mpi` feature gate).
* Random quantities can be generated via [`rand`](https://crates.io/crates/rand) (behind the `rand` feature gate, see the official documentation for more info).

## Design
Diman aims to make it as easy as possible to add compile-time unit safety to Rust code. Physical quantities are represented by the `Quantity<S, D>` struct, where `S` is the underlying storage type (`f32`, `f64`, ...) and `D` is the  dimension of the quantity. For example, in order to represent the [SI system of units](https://www.nist.gov/pml/owm/metric-si/si-units), the dimension type would look as follows:
```rust
#![feature(adt_const_params)]
use diman::dimension;

#[dimension]
pub struct Dimension {
    pub length: i32,
    pub time: i32,
    pub mass: i32,
    pub temperature: i32,
    pub current: i32,
    pub amount_of_substance: i32,
    pub luminous_intensity: i32,
}
```
Addition and subtraction of two quantities is only allowed when the `D` type is the same. During multiplication of two quantities, all the entries of the two dimension are added. This functionality is implemented automatically by the `#[diman]` macro. 
Note: This macro is currently not a derive macro for the Mul/Div traits because the dimension multiplication and division are required to be const. While this is possible, it would require using yet another unstable feature [`const_trait_impl`](https://github.com/rust-lang/rust/issues/67792).

Using the above `Dimension` type, we can define our own quantity type and a corresponding set of physical quantities and dimensions using the `unit_system` macro:


```rust ignore
#![allow(incomplete_features)]
#![feature(generic_const_exprs, adt_const_params)]
use diman::unit_system;
use diman::dimension;

#[dimension]
pub struct Dimension {
    pub length: i32,
    pub time: i32,
}

unit_system!(
    Quantity,
    Dimension,
    [
        def Length = { length: 1 },
        def Time = { time: 1 },
        def Velocity = Length / Time,
        unit (meters, "m") = Length,
        unit (kilometers, "km") = 1000.0 * meters,
        unit (seconds, "s") = 1.0 * Time,
        unit hours = 3600 * seconds,
        unit meters_per_second = meters / seconds,
        unit kilometers_per_hour = kilometers / hours,
        constant MY_FAVORITE_VELOCITY = 1000 * meters_per_second,
    ]
);

use f64::{Length, Time, Velocity, MY_FAVORITE_VELOCITY};

fn fast_enough(x: Length, t: Time) {
    let vel = x / t;
    if vel > MY_FAVORITE_VELOCITY {
        println!("{} is definitely fast enough!", vel.in_kilometers_per_hour());
    }
}

fast_enough(Length::kilometers(100.0), Time::hours(0.3));
```

This will define the `Quantity` type and implement all the required traits and methods.
Here, `def` defines Quantities, which are concrete types, `unit` defines units, which are methods on the corresponding quantities and `constant` defines constants. The macro also accepts more complex definitions such as `def EnergyRatePerVolume = (Energy / Time) / Volume`.
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

## Quantity products and quotients
Sometimes, intermediate types in computations are quantities that doesn't really have a nice name and are also
not needed too many times. Having to add a definition to the unit system for this case can be cumbersome.
This is why the `Product` and `Quotient` types are provided:
```rust
use diman::si::f64::{Length, Time, Velocity, Area};
use diman::{Product, Quotient};
let x: Product<(Length, Time)> = Length::meters(10.0) * Time::seconds(2.0);
let y: Product<(Length, Time, Velocity)> = Area::square_meters(5.0);
let z: Quotient<Length, Time> = Length::meters(10.0) / Time::seconds(2.0);
```
