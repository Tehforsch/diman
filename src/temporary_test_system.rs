use derive_dimension::diman_dimension;

use crate as diman;

#[derive(PartialEq, Eq, Debug, Clone)]
#[diman_dimension]
pub struct Dimension {
    pub length: i32,
}

::unit_system::unit_system_2!(Dimension, Quantity, [
        Length = {
            dimension: { length: 1 },
            units: [
                { name: meters, factor: 1.0, symbol: "m" }
            ]
        },
]);
