use derive_dimension::diman_dimension;

use crate as diman;

#[derive(PartialEq, Eq, Debug, Clone)]
#[diman_dimension]
pub struct Dimension {
    pub length: i32,
}

::unit_system::unit_system_2!(UNIT_NAMES, Dimension, Quantity, []);
