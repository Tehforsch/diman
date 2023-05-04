pub type Product<Q1, Q2> = <Q1 as std::ops::Mul<Q2>>::Output;
pub type Quotient<Q1, Q2> = <Q1 as std::ops::Div<Q2>>::Output;

#[cfg(test)]
#[cfg(any(feature = "default-f32", feature = "default-f64"))]
mod tests {
    use crate::si::Length;
    use crate::si::Time;
    use crate::type_aliases::Quotient;

    #[test]
    fn quotient_type_alias() {
        fn velocity(length: Length, time: Time) -> Quotient<Length, Time> {
            length / time
        }
        velocity(Length::meters(5.0), Time::seconds(2.0));
    }
}
