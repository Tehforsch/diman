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

    // ($n:ident, $check_severity:expr, $($x:ident : $type:ty),+, $code:block) => {
macro_rules! impl_qproduct {
    ($f: ident, $($fs: ident),+) => {
        impl<$f, $($fs),+> QProduct for ($f, $($fs),+)
        where
            ($($fs),+): QProduct,
            $f: QProduct,
            <($($fs),+) as QProduct>::Output: std::ops::Mul<<$f as QProduct>::Output>,
        {
            type Output =
                <<($($fs),+) as QProduct>::Output as std::ops::Mul<<$f as QProduct>::Output>>::Output;
        }

    }
}

impl<Q1, Q2> QProduct for (Q1, Q2)
where
    Q1: QProduct,
    Q2: QProduct,
    <Q1 as QProduct>::Output: std::ops::Mul<<Q2 as QProduct>::Output>,
{
    type Output = <<Q1 as QProduct>::Output as std::ops::Mul<<Q2 as QProduct>::Output>>::Output;
}

impl_qproduct!(Q1, Q2, Q3);
impl_qproduct!(Q1, Q2, Q3, Q4);
impl_qproduct!(Q1, Q2, Q3, Q4, Q5);
impl_qproduct!(Q1, Q2, Q3, Q4, Q5, Q6);
impl_qproduct!(Q1, Q2, Q3, Q4, Q5, Q6, Q7);
impl_qproduct!(Q1, Q2, Q3, Q4, Q5, Q6, Q7, Q8);
impl_qproduct!(Q1, Q2, Q3, Q4, Q5, Q6, Q7, Q8, Q9);
impl_qproduct!(Q1, Q2, Q3, Q4, Q5, Q6, Q7, Q8, Q9, Q10);
impl_qproduct!(Q1, Q2, Q3, Q4, Q5, Q6, Q7, Q8, Q9, Q10, Q11);
impl_qproduct!(Q1, Q2, Q3, Q4, Q5, Q6, Q7, Q8, Q9, Q10, Q11, Q12);
impl_qproduct!(Q1, Q2, Q3, Q4, Q5, Q6, Q7, Q8, Q9, Q10, Q11, Q12, Q13);
impl_qproduct!(Q1, Q2, Q3, Q4, Q5, Q6, Q7, Q8, Q9, Q10, Q11, Q12, Q13, Q14);
impl_qproduct!(Q1, Q2, Q3, Q4, Q5, Q6, Q7, Q8, Q9, Q10, Q11, Q12, Q13, Q14, Q15);
impl_qproduct!(Q1, Q2, Q3, Q4, Q5, Q6, Q7, Q8, Q9, Q10, Q11, Q12, Q13, Q14, Q15, Q16);
