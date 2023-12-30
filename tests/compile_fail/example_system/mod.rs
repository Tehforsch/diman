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
    unit meters: Length;
    unit kilometers = 1000.0 * meters;
    #[base(Time)]
    unit seconds: Time;
    dimension Velocity = Length / Time;
    unit meters_per_second = meters / seconds;
    dimension Energy = Mass * Velocity * Velocity;
    unit joules: Energy = kilograms * meters_per_second^2;
    #[base(Mass)]
    unit kilograms: Mass;
    unit grams = 1e-3 * kilograms;
    dimension Area = Length * Length;
    dimension Volume = Length * Length * Length;
    dimension Force = Energy / Length;
    unit newtons = joules / meters;
    constant SOLAR_MASS = 1.988477e30 * kilograms;
    constant SOLAR_MASS_GRAMS = 1.988477e33 * grams;
    constant SOLAR_MASS_AWKWARD = 1.988477e30 * kilograms / (seconds / seconds);
);
