pub(crate) mod dimension;

pub use dimension::Dimension;
use dimension::NONE;

use crate::define_system;
use crate::unit_system;
use ::unit_system::unit_system_2;

define_system!(Quantity, Dimension, NONE, UNIT_NAMES);

#[rustfmt::skip]
unit_system!(
    Dimension,
    Quantity,
    NONE,
    UNIT_NAMES,
    DIMENSIONLESS, Dimensionless, length: 0,
    {
        dimensionless, 1.0, ""
    },
    LENGTH, Length, length: 1,
    {
        meters, 1.0, "m",
        kilometers, 1000.0, "km",
        astronomical_units, 1.495978707e11, "au"
    },
    AREA, Area, length: 2,
    {
    },
    TIME, Time, time: 1,
    {
        seconds, 1.0, "s",
        hours, 3600.0, "h",
        years, 31557600.0, "yr"
    },
    VELOCITY, Velocity, length: 1, time: -1,
    {
        meters_per_second, 1.0, "m/s",
        astronomical_units_per_day, 1731460.0, "au/d"
    },
    MASS, Mass, mass: 1,
    {
        kilograms, 1.0, "kg",
        earth, 5.9722e24, "Mearth",
        solar, 1.988477e30, "Msol"
    },
    ACCELERATION, Acceleration, length: 1, time: -2,
    {
        meters_per_second_squared, 1.0, "m/s^2"
    },
    FORCE, Force, mass: 1, length: 1, time: -2,
    {
        newtons, 1.0, "N"
    },
    ENERGY, Energy, mass: 1, length: 2, time: -2,
    {
        joules, 1.0, "J"
    },
    DENSITY, Density, mass: 1, length: -2, time: 0,
    {
        kilogram_per_square_meter, 1.0, "kg/m^2"
    },
    VOLUME, Volume, mass: 0, length: 3, time: 0,
    {
    },
    PRESSURE, Pressure, mass: 1, length: -1, time: -2,
    {
        pascals, 1.0, "Pa"
    },
    ENTROPY, Entropy, mass: 1, length: 2, time: -2, temperature: -1,
    {
    },
    ENTROPIC_FUNCTION, EntropicFunction, length: 4, mass: -1, time: 2,
    {
    },
    NUMBERDENSITY3D, NumberDensity3D, length: -3,
    {
    },
    NUMBERDENSITY2D, NumberDensity2D, length: -2,
    {
    },
    LENGTHMASS, LengthMass, mass: 1, length: 1,
    {
    },
    INVERSE_TIME, InverseTime, time: -1,
    {
    },
    INVERSE_TIME_SQUARED, InverseTimeSquared, time: -2,
    {
    }
    );

unit_system_2!(
    Dimension,
    Quantity,
    [
        Dimensionless = {
            dimension: {
                length: 0
            }
        },
        Length = { dimension: { length: 1 } },
        Dimensionless = {
            dimension: { length: 0 },
            units: [
                { name: meters, factor: 1.0, symbol: "m", prefixes: [k, m] }
            ]
        },
        // Area = { dimension: (Length, Length) }
    ]
);
