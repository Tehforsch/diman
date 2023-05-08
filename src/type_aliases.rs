pub trait QProduct {
    type Output;
}

pub type Product<Q1> = <Q1 as QProduct>::Output;
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

#[cfg(test)]
#[cfg(any(feature = "default-f32", feature = "default-f64"))]
mod tests {
    use crate::si::Length;
    use crate::si::Volume;
    use crate::si::Area;
    use crate::si::Velocity;
    use crate::si::Time;
    use crate::type_aliases::Product;
    use crate::type_aliases::Quotient;

    // These just need to compile

    fn _product_1(length: Length, time: Time) -> Product<(Length, Time)> {
        length * time
    }

    fn _quotient_1(length: Length, time: Time) -> Quotient<Length, Time> {
        length / time
    }

    fn _quotient_2(length: Length, time: Time) -> Quotient<(Length, Time), (Length, Velocity)> {
        let vel: Velocity = length / time;
        length * time / (length * vel)
    }

    fn _quotient_3(length: Length, time: Time) -> Quotient<(Length, Time), (Area, Volume)> {
        let vol: Volume = length.cubed();
        let area: Area = length.squared();
        length * time / (area * vol)
    }
}
