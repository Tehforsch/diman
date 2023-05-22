pub(crate) mod dimension;

pub use dimension::Dimension;
use dimension::NONE;

use crate::define_system;
use crate::unit_system;
use ::unit_system::unit_system_2;

// The macro will import things from diman::* which would not exist in this scope.
use crate as diman;

unit_system_2!(
    UNIT_NAMES,
    Dimension,
    Quantity,
    [
        Dimensionless = {
            dimension: {
            },
            units: [
                { name: dimensionless, factor: 1.0, symbol: "dimensionless" }
            ]
        },
        Length = {
            dimension: { length: 1 },
            units: [
                { name: meters, factor: 1.0, symbol: "m", prefixes: [k, m] },
                { name: kilometers, factor: 1000.0, symbol: "km", prefixes: [k, m] }
            ]
        },
        Time = {
            dimension: { time: 1 },
            units: [
                { name: seconds, factor: 1.0, symbol: "s" }
            ]
        },
        Velocity = {
            dimension: { length: 1, time: -1 },
            units: [
                { name: meters_per_second, factor: 1.0, symbol: "m/s" }
            ],
        },
        Energy = {
            dimension: { length: 2, time: -2, mass: 1 },
            units: [
                { name: joules, factor: 1.0, symbol: "J" },
            ],
        },
        Mass = {
            dimension: { mass: 1 },
            units: [
                { name: kilograms, factor: 1.0, symbol: "kg" },
            ],
        },
        Area = {
            dimension: { length: 2 },
        },
        Volume = {
            dimension: { length: 3 },
        },
        Force = {
            dimension: { length: 1, time: -2, mass: 1 },
            units: [
                { name: newtons, factor: 1.0, symbol: "N" }
            ]
        },
    ]
);

// define_system!(Quantity, Dimension, NONE, UNIT_NAMES);
