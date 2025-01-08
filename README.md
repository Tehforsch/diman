<!-- cargo-rdme start -->

Diman is a library for zero-cost compile time unit checking.

```rust
use diman::si::dimensions::{Length, Time, Velocity};
use diman::si::units::{seconds, meters, kilometers, hours, hour};

fn get_velocity(x: Length<f64>, t: Time<f64>) -> Velocity<f64> {
    x / t
}

let v1 = get_velocity(36.0 * kilometers, 1.0 * hours);
let v2 = get_velocity(10.0 * meters, 1.0 * seconds);

assert_eq!(v1, v2);
assert_eq!(format!("{} km/h", v1.value_in(kilometers / hour)), "36 km/h");
```

Diman prevents unit errors at compile time:
```rust
let time = 1.0 * seconds;
let length = 10.0 * meters;
let sum = length + time;
```
This results in a compiler error:
```text
let sum = length + time;
                   ^^^^
= note: expected struct `Quantity<_, Dimension { length: 1, time: 0, mass: 0, temperature: 0, current: 0, amount_of_substance: 0, luminous_intensity: 0 }>`
        found struct `Quantity<_, Dimension { length: 0, time: 1, mass: 0, temperature: 0, current: 0, amount_of_substance: 0, luminous_intensity: 0 }>`
```

While Diman provides a full definition of the SI system of units, it also fully supports defining custom systems of dimensions and units:
```rust
diman::unit_system!(
    quantity_type Quantity;
    dimension_type Dimension;

    dimension Information;
    dimension Time;

    dimension Bandwidth = Information / Time;

    #[prefix(kilo, mega)]
    #[base(Information)]
    #[symbol(B)]
    unit byte;

    #[base(Time)]
    #[symbol(s)]
    unit seconds;

    unit hours = 3600 * seconds;
);

fn get_bitrate(information: Information<f64>, time: Time<f64>) -> Bandwidth<f64> {
    information / time
}

get_bitrate(50.0 * megabyte, 2.0 * hours);
```

# Disclaimer
Diman is implemented using Rust's const generics feature. This makes for very readable error messages compared to alternatives which are typically based on `typenum`. While `min_const_generics` has been stabilized since Rust 1.51, Diman uses more complex generic expressions and therefore requires the two currently unstable features `generic_const_exprs` and `adt_const_params`.

Moreover, Diman is in its early stages of development and APIs might change.

If you cannot use unstable Rust for your project, consider using [`uom`](https://crates.io/crates/uom) or [`dimensioned`](https://crates.io/crates/dimensioned), both of which do not require any experimental features.

# Features
* Invalid operations between physical quantities (adding length and time, for example) turn into compile errors.
* Newly created quantities are automatically converted to an underlying base representation. This means that the used types are dimensions (such as `Length`) instead of concrete units (such as `meters`) which makes for more meaningful code.
* Systems of dimensions and units can be user defined via the `unit_system!` macro. This gives the user complete freedom over the choice of dimensions and makes them part of the user's library, so that arbitrary new methods can be implemented on them.
* The `rational-dimensions` features allows the usage of quantities and units with rational exponents.
* `f32` and `f64` float storage types (behind the `f32` and `f64` feature gate respectively).
* The `std` feature is enabled by default. If disabled, Diman will be a `no_std` crate, thus suitable for use on embedded devices such as GPU device kernels.
* The `num-traits-libm` feature uses [libm](https://crates.io/crates/libm) to provide math functions in `no_std` environments. While one can use libm in `std`, the libm implementations are generally slower so this is unlikely to be desirable.
* Vector storage types via [`glam`](https://crates.io/crates/glam/) (behind the `glam-vec2`, `glam-vec3`, `glam-dvec2` and `glam-dvec3` features).
* Serialization and Deserialization via [`serde`](https://crates.io/crates/serde) (behind the `serde` feature gate, see the official documentation for more info).
* HDF5 support using [`hdf5-rs`](https://crates.io/crates/hdf5-rs/) (behind the `hdf5` feature gate).
* Quantities implement the `Equivalence` trait so that they can be sent via MPI using [`mpi`](https://crates.io/crates/mpi) (behind the `mpi` feature gate).
* Random quantities can be generated via [`rand`](https://crates.io/crates/rand) (behind the `rand` feature gate, see the official documentation for more info).

# The `Quantity` type
Physical quantities are represented by the `Quantity<S, D>` struct, where `S` is the underlying storage type (`f32`, `f64`, ...) and `D` is the  dimension of the quantity.
`Quantity` should behave like its underlying storage type whenever allowed by the dimensions.

## Arithmetics and math
Addition and subtraction of two quantities is allowed if the dimensions match:
```rust
let l = 5.0 * meters + 10.0 * kilometers;
```
Multiplication and division of two quantities produces a new quantity:
```rust
let l = 5.0 * meters;
let t = 2.0 * seconds;
let v: Velocity<f64> = l / t;
```
Addition and subtraction of a `Quantity` and a storage type is possible if and only if `D` is dimensionless:
```rust
let l1 = 5.0 * meters;
let l2 = 10.0 * kilometers;
let x = l1 / l2 - 0.5;
let y = 0.5 - l1 / l2;
```
`Quantity` implements the dimensionless methods of `S`, such as `sin`, `cos`, etc. for dimensionless quantities:
```rust
let l1 = 5.0f64 * meters;
let l2 = 10.0f64 * kilometers;
let angle_radians = (l1 / l2).asin();
```
Exponentiation and related operations are supported via `squared`, `cubed`, `powi`, `sqrt`, `cbrt`:
```rust
let length = 2.0f64 * meters;
let area = length.squared();
assert_eq!(area, 4.0 * square_meters);
assert_eq!(area.sqrt(), length);
let vol = length.cubed();
assert_eq!(vol, 8.0 * cubic_meters);
assert_eq!(vol.cbrt(), length);
let foo = length.powi::<4>();
```
Note that unlike its float equivalent, `powi` receives its exponent as a generic instead of as a normal function argument. Exponentiation of dimensionful quantities with an non-constant integer is not supported, since the compiler cannot infer the dimension of the return type. However, dimensionless quantities can be raised to arbitrary powers using `powf`:
```rust
let l1 = 2.0f64 * meters;
let l2 = 5.0f64 * kilometers;
let x = (l1 / l2).powf(2.71);
```
## Creation and conversion
New quantities can be created either by multiplying with a unit, or by calling the `.new` function on the unit:
```rust
let l1 = 2.0 * meters;
let l2 = meters.new(2.0);
assert_eq!(l1, l2);
```
For a full list of the units supported by dimans `SI` module, see [the definitions](src/si.rs).
Composite units can be defined on the spot via multiplication/division of units:
```rust
let v1 = (kilometers / hour).new(3.6);
let v2 = 3.6 * kilometers / hour;
assert_eq!(v1, 1.0 * meters_per_second);
assert_eq!(v2, 1.0 * meters_per_second);
```
Note that at the moment, the creation of quantities via units defined in this composite way incurs
a small performance overhead compared to creation from just a single unit (which is just a single multiplication). This will be fixed once [const_fn_floating_point_arithmetic](https://github.com/rust-lang/rust/issues/57241) or a similar feature is stabilized.

Conversion into the underlying storage type can be done using the `value_in` function:
```rust
let length = 2.0f64 * kilometers;
assert_eq!(format!("{} m", length.value_in(meters)), "2000 m");
```
This also works for composite units:
```rust
let vel = 10.0f64 * meters_per_second;
assert_eq!(format!("{} km/h", vel.value_in(kilometers / hour)), "36 km/h");
```
For dimensionless quantities, `.value()` provides access to the underlying storage types. Alternatively, dimensionless quantities also implement `Deref` for the same operation.
```rust
let l1: Length<f64> = 5.0 * meters;
let l2: Length<f64> = 10.0 * kilometers;
let ratio_value: f64 = (l1 / l2).value();
let ratio_deref: f64 = *(l1 / l2);
assert_eq!(ratio_value, ratio_deref);
```
## Unchecked creation and conversion
If absolutely required, `.value_unchecked()` provides access to the underlying storage type for all quantities. This is **not unit-safe** since the return value will depend on the unit system!
```rust
let length: Length<f64> = 5.0 * kilometers;
let value: f64 = length.value_unchecked();
assert_eq!(value, 5000.0); // This only holds in SI units!
```
Similarly, if absolutely required, new quantities can be constructed from storage types using `Quantity::new_unchecked`. This operation is also **not unit-safe**!
```rust
let length: Length<f64> = Length::new_unchecked(5000.0);
assert_eq!(length, 5.0 * kilometers); // This only holds in SI units!
```
The combination of `value_unchecked` and `new_unchecked` comes in handy when using third party libraries that only takes the raw storage type as argument. As an example, suppose we have a function `foo` that takes a `Vec<f64>` and returns a `Vec<f64>`, and suppose it sorts the numbers or does some other unit safe operation. Then we could reasonably write:
```rust
   let lengths: Vec<Length<f64>> = vec![
       1.0 * meters,
       2.0 * kilometers,
       3.0 * meters,
       4.0 * kilometers,
   ];
   let unchecked = lengths.into_iter().map(|x| x.value_unchecked()).collect();
   let fooed = foo(unchecked);
   let result: Vec<_> = fooed
       .into_iter()
       .map(|x| Length::new_unchecked(x))
       .collect();
```
## Debug
`Debug` is implemented and will print the quantity in its base representation.
```rust
let length: Length<f64> = 5.0 * kilometers;
let time: Time<f64> = 1.0 * seconds;
assert_eq!(format!("{:?}", length / time), "5000 m s^-1")
```

# Custom unit systems
## The `unit_system` macro
Diman also provides the `unit_system` macro for defining custom
unit systems for everything that is not covered by SI alone. The
macro will add a new quantity type and implement all the required
methods and traits to make it usable.
As an example, consider the following macro call:
```rust
diman::unit_system!(
    quantity_type Quantity;
    dimension_type Dimension;

    dimension Length;
    dimension Time;
    dimension Mass;

    dimension Velocity = Length / Time;
    dimension Frequency = 1 / Time;
    dimension Energy = Mass * Velocity^2;

    #[prefix(kilo, milli)]
    #[base(Length)]
    #[symbol(m)]
    unit meters;

    #[base(Time)]
    #[symbol(s)]
    unit seconds;

    unit hours: Time = 3600 * seconds;
    unit meters_per_second: Velocity = meters / seconds;
    unit kilometers_per_hour: Velocity = kilometers / hours;
    constant SPEED_OF_LIGHT = 299792458 * meters_per_second;
);


fn too_fast(x: Length<f64>, t: Time<f64>) -> bool {
    x / t > 0.1f64 * SPEED_OF_LIGHT
}

too_fast(100.0 * kilometers, 0.3 * hours);
```

The macro accepts five different keywords:
1. `quantity_type` specifies the name of the quantity type. Required for compiler error messages to have something to point to.
2. `dimension_type` specifies the name of the dimension type. Required for compiler error messages to have something to point to.
3. `dimension` defines a new dimension which is a type. Dimensions without a right hand side are base dimensions (such as `Length` and `Time` in this example), whereas dimensions with a right hand side are derived dimensions (such as `Velocity` in this example).
4. `unit` defines a new units, which are methods on the corresponding quantities and `constant` defines constants. Units without a right-hand side are the base units to one specific base dimension, meaning that they are the unit that will internally be represented with a conversion factor of 1. Base units require the `#[base(...)]` attribute in order to specify which dimension they are the base unit of. Units with a right hand side are derived from other units.
5. `constant` defines a new constant.

## SI Prefixes
Unit prefixes can automatically be generated with the `#[prefix(...)]` attribute for unit statements.
For example
```rust
#[base(Length)]
#[prefix(kilo, milli)]
#[symbol(m)]
unit meters;
```
will automatically generate the unit `meters` with symbol `m`, as well as `kilometers` and `millimeters` with symbols `km` and `mm` corresponding to `1e3 m` and `1e-3 m`.
For simplicity, the attribute `#[metric_prefixes]` is provided, which will generate all metric prefixes from `atto-` to `exa-` automatically.

## Aliases
Unit aliases can automatically be generated with the `#[alias(...)]` macro. For example
```rust
#[alias(metres)]
unit meters;
```
will automatically generate a unit `metres` that has exactly the same definition as `meters`. This works with prefixes as expected (i.e. an alias is generated for every prefixed unit).

# Quantity products and quotients
Sometimes, intermediate types in computations are quantities that don't really have a nice name and are also
not needed too many times. Having to add a definition to the unit system for this case can be cumbersome.
This is why the `Product` and `Quotient` types are provided:
```rust
use diman::si::dimensions::{Length, Time};
use diman::{Product, Quotient};

fn foo(l: Length<f64>, t: Time<f64>) -> Product<Length<f64>, Time<f64>> {
    l * t
}

fn bar(l: Length<f64>, t: Time<f64>) -> Quotient<Length<f64>, Time<f64>> {
    l / t
}
```

# Rational dimensions
The `rational-dimensions` feature allows using quantities with rational exponents in their base dimensions, as opposed to just integer values. This allows expressing defining dimensions and units such as:
```rust
unit_system!(
    // ...
    dimension Sorptivity = Length Time^(-1/2);
    unit meters_per_sqrt_second: Sorptivity = meters / seconds^(1/2);
    // ...
);
let l = 2.0 * micrometers;
let t = 5.0 * milliseconds;
let sorptivity: Sorptivity = l / t.sqrt();
```

The unit system generated with `rational-dimensions` supports a superset of features of a unit system generated without them.
Still, this feature should be enabled only when necessary, since the compiler errors in case of dimension mismatches will be harder to read.

# `serde`
Serialization and deserialization of the units is provided via [`serde`](https://crates.io/crates/serde) if the `serde` feature gate is enabled:
```rust
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Parameters {
    my_length: Length<f64>,
    my_vel: Velocity<f64>,
}

let params: Parameters =
     serde_yaml::from_str("
        my_length: 100 m
        my_vel: 10 m s^-1
    ").unwrap();
assert_eq!(
    params,
    Parameters {
        my_length: 100.0 * meters,
        my_vel: 10.0 * meters_per_second,
    }
)
```

# `rand`
Diman allows generating random quantities via [`rand`](https://crates.io/crates/rand) if the `rand` feature gate is enabled:
```rust

let mut rng = rand::thread_rng();
for _ in 0..100 {
    let start = 0.0 * meters;
    let end = 1.0 * kilometers;
    let x = rng.gen_range(start..end);
    assert!(Length::meters(0.0) <= x);
    assert!(x < Length::meters(1000.0));
}
```

<!-- cargo-rdme end -->
