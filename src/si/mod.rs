use crate as diman;
use crate::diman_dimension;
use crate::unit_system;

#[diman_dimension]
pub struct Dimension {
    pub length: i32,
    pub time: i32,
    pub mass: i32,
    pub temperature: i32,
    pub current: i32,
    pub amount_of_substance: i32,
    pub luminous_intensity: i32,
}

unit_system!(
    Dimension,
    Quantity,
    [
        def Dimensionless = {},
        unit dimensionless = Dimensionless,
        def Length = { length: 1 },
        unit (meters, "m") = Length,
        unit (kilometers, "km") = 1000.0 * meters,
        def Time = { time: 1 },
        unit (seconds, "s") = 1.0 * Time,
        unit (hours, "h") = 3600 * seconds,
        def Velocity = Length / Time,
        unit (meters_per_second, "m/s") = meters / seconds,
        def Energy = Mass * Velocity * Velocity,
        unit (joules, "J") = 1.0 * Energy,
        def Mass = { mass: 1 },
        unit (kilograms, "kg") = Mass,
        unit (grams, "g") = 1e-3 * kilograms,
        def Area = Length * Length,
        def Volume = Length * Length * Length,
        def Force = Energy / Length,
        unit (newtons, "N") = joules / meters,
        constant SOLAR_MASS = 1.988477e30 * kilograms,
        constant SOLAR_MASS_GRAMS = 1.988477e33 * grams,
        constant SOLAR_MASS_AWKWARD = 1.988477e30 * kilograms / (seconds / seconds),
    ]
);
