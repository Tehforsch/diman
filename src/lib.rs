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
pub use diman_unit_system::unit_system;
pub use type_aliases::Product;
pub use type_aliases::QProduct;
pub use type_aliases::Quotient;
