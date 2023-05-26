use crate as diman;
use crate::dimension;
use crate::unit_system;

#[dimension]
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
    Quantity,
    Dimension,
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
        def Area = Length * Length,
        unit (square_meters, "m^2") = meters * meters,
        def Volume = Length * Length * Length,
        unit (cubic_meters, "m^3") = meters * meters * meters,
    ]
);
