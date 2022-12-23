# Diman
Diman is a library for zero-cost compile time unit checking.

```rust ignore
use diman::si::{Length, Time, Velocity};

fn get_velocity(x: Length, t: Time) -> Velocity {
    x / t
}

let v1 = get_velocity(Length::kilometers(36.0), Time::hours(1.0));
let v2 = get_velocity(Length::meters(10.0), Time::seconds(1.0));

assert!(((v1 - v2) / (v1 + v2)).value() < 1e-10);
```

Let's try to assign add quantities with incompatible units:
```rust compile_fail
use diman::si::{Length, Time};

let time = Time::seconds(1.0);
let length = Length::meters(10.0);
let sum = length + time;
```
This results in a compiler error:
```text
let sum = length + time;
                   ^^^^
= note: expected struct `Quantity<_, Dimension { length: 1, time: 0, mass: 0, temperature: 0 }>`
        found struct `Quantity<_, Dimension { length: 0, time: 1, mass: 0, temperature: 0 }>`
```


## Disclaimer
Diman is implemented using Rust's const generics feature. While `min_const_generics` has been stabilized since Rust 1.51, Diman uses more complex generic expressions and therefore requires the two currently unstable features `generic_const_exprs` and `adt_const_params`. 

Moreover, Diman is in its early stages of development and APIs will change.

If you cannot use unstable Rust for your project or require a stable library for your project, consider using [`uom`](https://crates.io/crates/uom) or [`dimensioned`](https://crates.io/crates/dimensioned), both of which do not require any experimental features and are much more mature libraries in general.

## Features
* Newly created quantities are automatically converted to an underlying base representation.
* Invalid operations between physical quantities turn into compile errors.
* Systems of units and quantities can be user defined via the `unit_system!` macro. This gives the user complete freedom over the choice of quantities and makes them part of the user's library, so that arbitrary new methods can be implemented on them.
* Vector storage types via [`glam`](https://crates.io/crates/glam/).
* Serialization and Deserialization via [`serde`](https://crates.io/crates/serde) (behind the `serde` feature).
* HDF5 support using [`hdf5-rs`](https://crates.io/crates/hdf5-rs/) (behind the `hdf5` feature).
* Quantities can be sent via MPI using [`mpi`](https://crates.io/crates/mpi) (behind the `mpi` feature).
* The default float storage type can be chosen via the `default-f32`, `default-f64` features. For example, if the `default-f64` is activated, `Length::meters(1.0)` would result in a length represented by a `f64`, but `F32Length::meters(1.0)` would still work as expected.

## Design
Diman aims to make it as easy as possible to add compile-time unit safety to Rust code. Physical quantities are represented by the `Quantity<S, D>` struct, where `S` is the underlying storage type (`f32`, `f64`, ...) and `D` is the  dimension of the quantity. For example, in order to represent the [SI system of units](https://www.nist.gov/pml/owm/metric-si/si-units), the type D would look as follows:
```rust
#[derive(PartialEq, Eq, Debug, Clone)]
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
Addition and subtraction of two quantities is only allowed when the `D` type is the same. During multiplication of two quantities, all the entries of the two dimension are added.
