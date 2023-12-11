use ::diman::unit_system;


unit_system!(
    quantity_type Quantity,
    dimension_type Dimension,
    dimension Length,
    dimension Time,
    dimension Mass,
    dimension Temperature,
    dimension Dimensionless = {},
    unit dimensionless = Dimensionless,
    unit (meters, "m") = Length,
    unit (kilometers, "km") = 1000.0 * meters,
    unit (seconds, "s") = 1.0 * Time,
    dimension Velocity = Length / Time,
    unit (meters_per_second, "m/s") = meters / seconds,
    dimension Energy = Mass * Velocity * Velocity,
    unit (joules, "J") = 1.0 * Energy,
    unit (kilograms, "kg") = Mass,
    unit (grams, "g") = 1e-3 * kilograms,
    dimension Area = Length * Length,
    dimension Volume = Length * Length * Length,
    dimension Force = Energy / Length,
    unit (newtons, "N") = joules / meters,
    constant SOLAR_MASS = 1.988477e30 * kilograms,
    constant SOLAR_MASS_GRAMS = 1.988477e33 * grams,
    constant SOLAR_MASS_AWKWARD = 1.988477e30 * kilograms / (seconds / seconds),
);
