# Diman
Diman is a library for zero-cost compile time unit checking.

```rust
#![feature(generic_const_exprs)]

use diman::si::{Length, Time, Velocity};

fn get_velocity(x: Length, t: Time) -> Velocity {
    x / t
}

let v1 = get_velocity(Length::kilometers(36.0), Time::hours(1.0));
let v2 = get_velocity(Length::meters(10.0), Time::seconds(1.0));

assert!(((v1 - v2) / (v1 + v2)).value() < 1e-10);
```

## Disclaimer
Diman is implemented using Rust's const generics feature. While `min_const_generics` has been stabilized since Rust 1.51, Diman uses more complex generic expressions and therefore requires the two currently unstable features `generic_const_exprs` and `adt_const_params`. 

Moreover, Diman is in its early stages of development and APIs will change.

If you cannot use unstable Rust for your project or require a stable library for your project, consider using [`uom`](https://crates.io/crates/uom) or [`dimensioned`](https://crates.io/crates/dimensioned), both of which do not require any experimental features and are much more mature libraries in general.
