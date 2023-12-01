use ::diman::unit_system;

unit_system!(
    Quantity,
    Dimension {
        Length,
        Time,
        Mass,
        Temperature,
    },
    [
        def Dimensionless = {},
        unit dimensionless = Dimensionless,
        def Length = { length: 1 },
        unit (meters, "m") = Length,
        unit (kilometers, "km") = 1000.0 * meters,
        def Time = { time: 1 },
        unit (seconds, "s") = 1.0 * Time,
        def Velocity = Length / Time,
        unit (meters_per_second, "m/s") = meters / seconds,
        def Energy = Mass * Velocity * Velocity,
        unit (joules, "J") = 1.0 * Energy,
        def Mass = { mass: 1 },
        unit (kilograms, "kg") = Mass,
        unit (grams, "g") = 1e-3 * kilograms,
        def Area = Length^2,
        unit square_meters = meters^2,
        def Volume = Length^3,
        unit cubic_meters = meters^3,
        def Force = Energy / Length,
        def Temperature = { temperature: 1 },
        unit (kelvins, "K") = Temperature,
        def InverseTemperature = 1 / Temperature,
        unit (newtons, "N") = joules / meters,
        constant SOLAR_MASS = 1.988477e30 * kilograms,
        constant SOLAR_MASS_GRAMS = 1.988477e33 * grams,
        constant SOLAR_MASS_AWKWARD = 1.988477e30 * kilograms / (seconds / seconds),
    ]
);
