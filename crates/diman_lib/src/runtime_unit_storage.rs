pub struct RuntimeUnit<'a, D> {
    pub symbol: &'a str,
    pub dimension: D,
    pub magnitude: f64,
}

impl<'a, D> RuntimeUnit<'a, D> {
    pub fn new(symbol: &'a str, dimension: D, magnitude: f64) -> Self {
        Self {
            symbol,
            dimension,
            magnitude,
        }
    }
}

pub struct RuntimeUnitStorage<'a, D> {
    units: &'a [RuntimeUnit<'a, D>],
}

impl<'a, D: PartialEq> RuntimeUnitStorage<'a, D> {
    pub fn new(units: &'a [RuntimeUnit<'a, D>]) -> Self {
        Self { units }
    }

    pub fn get_first_unit_for_dimension(&self, dim: D) -> Option<&'a RuntimeUnit<'a, D>> {
        self.units.iter().find(|unit| unit.dimension == dim)
    }

    pub fn get_unit_by_symbol(&self, symbol: &str) -> Option<&'a RuntimeUnit<'a, D>> {
        self.units.iter().find(|unit| unit.symbol == symbol)
    }
}
