pub(crate) mod dimension;

pub use dimension::Dimension;

use ::unit_system::unit_system;

// The macro will import things from diman::* which would not exist in this scope.
use crate as diman;

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
        def Velocity = Length / Time,
        unit (meters_per_second, "m/s") = meters / seconds,
        def Energy = Mass * Velocity * Velocity,
        unit (joules, "J") = 1.0 * Energy,
        def Mass = { mass: 1 },
        unit (kilograms, "kg") = Mass,
        def Area = Length * Length,
        def Volume = Length * Length * Length,
        def Force = Energy / Length,
        unit (newtons, "N") = joules / meters,
    ]
);
