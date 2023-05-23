use derive_dimension::diman_dimension;

use crate as diman;

#[derive(PartialEq, Eq, Debug, Clone)]
#[diman_dimension]
pub struct Dimension {
    pub length: i32,
}

::unit_system::unit_system!(Dimension, Quantity, [
    def Length = { length: 1 },
    def Shlami = Length,
    def Area = Length * Length,
    unit (square_meters, "m^2") = Area,
    unit (square_kilometers, "km^2") = 1e6 * square_meters,
    unit square_centimeters = 1e-4 * Area,
    unit (meters, "m", ["k", "m", "M"]) = Length,
]);
