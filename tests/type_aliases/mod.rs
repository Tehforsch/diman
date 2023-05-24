use crate::example_system::f64::Area;
use crate::example_system::f64::Length;
use crate::example_system::f64::Time;
use crate::example_system::f64::Velocity;
use crate::example_system::f64::Volume;
use diman::Product;
use diman::Quotient;

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
