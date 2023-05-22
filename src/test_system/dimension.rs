use derive_dimension::diman_dimension;

#[derive(PartialEq, Eq, Debug, Clone)]
#[diman_dimension]
pub struct Dimension {
    pub length: i32,
    pub time: i32,
    pub mass: i32,
    pub temperature: i32,
}
