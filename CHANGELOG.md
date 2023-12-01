# v0.3.0
## Features
- Add various trait implementations for dimensionless quantities and their underlying storage types: `MulAssign`, `DivAssign` , `PartialEq`, `PartialOrd`, `From`. For example, this makes it possible to write:
```rust
let x: f64 = 1.0;
let l1 = Length::meters(200.0);
let l2 = Length::kilometers(1.0);
let mut ratio = l1 / l2;
if l1 / l2 < x {
    ratio += x;
}
```
- Factors of one can now be used in quantity definitions (see #28):
```rust
def InverseTemperature = 1 / Temperature
```
- Exponents can now be used in quantity definitions(see #32):
```rust
def Volume = Length^3
```
- Added more SI dimensions and units (see #17). (@jedbrown)
- Added an example for conversions between gas equations of state (see #17). (@jedbrown)
- The `unit_system` macro now emits all the definitions it can, even if it is called with unresolvable or invalid definitions. This makes it easier to spot the error in the macro call in large codebases. Previously, hundreds of errors would be emitted due to one wrong definition, because every quantity and unit was now undefined.
- Automatically derive `ConstParamTy` for the `Dimension` type.

## Bugfixes
- Fixed proc macro panicking in cases with recursive / duplicate definitions of quantities. Instead, a proper error message is emitted that highlights the conflicting quantities. (See #21)
- Fixed a number of cases in which the compiler emitted extremely ugly error messages (see #23).
- Fix dimension expressions being parsed right-associatively (see #30)

## Other
- Added a large number of compile_fail tests as a way to ensure emitted error messages remain useful in future diman/compiler versions.

# v0.2.0
Initial release
