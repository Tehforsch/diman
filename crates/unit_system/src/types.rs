use syn::*;

pub struct DimensionInt {
    pub val: i32,
}

pub struct Factor {
    pub factor: f64,
}

pub struct Symbol {
    pub symbol: String,
}

pub struct Prefixes {
    pub prefixes: Vec<Ident>,
}

pub struct UnitEntry {
    pub name: Ident,
    pub factor: f64,
    pub symbol: Option<String>,
    pub prefixes: Vec<Ident>,
}

pub struct DimensionEntry {
    pub ident: Ident,
    pub value: DimensionInt,
}

pub struct UnitsEntry {
    pub units: Vec<UnitEntry>,
}

pub struct DimensionsEntry {
    pub fields: Vec<DimensionEntry>,
}

pub struct QuantityEntry {
    pub name: Ident,
    pub dimensions_def: DimensionsEntry,
    pub units_def: UnitsEntry,
}

pub struct Defs {
    pub unit_names_type: Type,
    pub dimension_type: Type,
    pub quantity_type: Type,
    pub quantities: Vec<QuantityEntry>,
}
