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
