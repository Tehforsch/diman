/// The trait enabling the `Product` and `Quotient` type aliases.
pub trait QProduct {
    type Output;
}

/// Constructs a product of quantities for one-off quantities.
/// ```
/// # use diman::si::f64::{Length, Time, Velocity, Area};
/// # use diman::Product;
/// let x: Product<(Length, Time)> = Length::meters(10.0) * Time::seconds(2.0);
/// let y: Product<(Length, Time, Velocity)> = Area::square_meters(5.0);
/// ```
pub type Product<Q1> = <Q1 as QProduct>::Output;

/// Constructs a quotient of two quantities for one-off quantities.
/// ```
/// # use diman::si::f64::{Length, Time};
/// # use diman::Quotient;
/// let x: Quotient<Length, Time> = Length::meters(10.0) / Time::seconds(2.0);
/// ```
pub type Quotient<Q1, Q2> =
    <<Q1 as QProduct>::Output as std::ops::Div<<Q2 as QProduct>::Output>>::Output;

impl<Q1: QProduct, Q2: QProduct> QProduct for (Q1, Q2)
where
    <Q1 as QProduct>::Output: std::ops::Mul<<Q2 as QProduct>::Output>,
{
    type Output = <<Q1 as QProduct>::Output as std::ops::Mul<<Q2 as QProduct>::Output>>::Output;
}

impl<Q1, Q2, Q3: QProduct> QProduct for (Q1, Q2, Q3)
where
    (Q1, Q2): QProduct,
    <(Q1, Q2) as QProduct>::Output: std::ops::Mul<<Q3 as QProduct>::Output>,
{
    type Output =
        <<(Q1, Q2) as QProduct>::Output as std::ops::Mul<<Q3 as QProduct>::Output>>::Output;
}

impl<Q1, Q2, Q3, Q4: QProduct> QProduct for (Q1, Q2, Q3, Q4)
where
    (Q1, Q2, Q3): QProduct,
    <(Q1, Q2, Q3) as QProduct>::Output: std::ops::Mul<<Q4 as QProduct>::Output>,
{
    type Output =
        <<(Q1, Q2, Q3) as QProduct>::Output as std::ops::Mul<<Q4 as QProduct>::Output>>::Output;
}
