use ::diman::unit_system;

unit_system!(
    quantity_type Quantity;
    dimension_type Dimension;
    dimension Length;
    dimension Time;
    dimension Mass;
    dimension Temperature;
    dimension Dimensionless = 1;
    unit dimensionless = 1;
    #[base(Length)]
    #[symbol(m)]
    #[metric_prefixes]
    unit meters: Length;
    #[base(Time)]
    #[symbol(s)]
    unit seconds: Time;
    dimension Velocity = Length / Time;
    unit meters_per_second = meters / seconds;
    dimension Energy = Mass * Velocity * Velocity;
    #[symbol(J)]
    unit joules = kilograms * meters_per_second^2;
    #[base(Mass)]
    #[symbol(kg)]
    unit kilograms: Mass;
    unit grams = 1e-3 * kilograms;
    dimension Area = Length^2;
    unit square_meters = meters^2;
    dimension Volume = Length^3;
    unit cubic_meters = meters^3;
    dimension Force = Energy / Length;
    #[base(Temperature)]
    #[symbol(K)]
    unit kelvins: Temperature;
    dimension InverseTemperature = 1 / Temperature;
    unit newtons = joules / meters;
    constant SOLAR_MASS = 1.988477e30 * kilograms;
    constant SOLAR_MASS_GRAMS = 1.988477e33 * grams;
    constant SOLAR_MASS_AWKWARD = 1.988477e30 * kilograms / (seconds / seconds);
);
