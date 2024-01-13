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

    pub fn get_first_symbol(&self, dim: D) -> Option<&'a str> {
        self.units
            .iter()
            .filter(|unit| unit.dimension == dim)
            .map(|unit| unit.symbol)
            .next()
    }
}
