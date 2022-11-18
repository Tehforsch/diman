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

