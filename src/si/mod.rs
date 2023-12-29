use crate as diman;
use crate::unit_system;

unit_system!(
    quantity_type Quantity;
    dimension_type Dimension;
    dimension Length;
    dimension Time;
    dimension Mass;
    dimension Temperature;
    dimension Current;
    dimension AmountOfSubstance;
    dimension LuminousIntensity;
    dimension Dimensionless = 1;
    #[base(Dimensionless)]
    unit dimensionless: Dimensionless;
    #[base(Length)]
    unit meters: Length;
    unit kilometers = 1000.0 * meters;
    #[base(Time)]
    #[metric_prefixes]
    unit seconds: Time;
    unit hours: Time = 3600 * seconds;
    dimension Velocity = Length / Time;
    unit meters_per_second = meters / seconds;
    dimension Area = Length * Length;
    unit square_meters = meters^2;
    dimension Volume = Length^3;
    unit cubic_meters = meters^3;
    #[base(Mass)]
    unit kilograms: Mass;
    unit grams = 1e-3 * kilograms;
    dimension MassDensity = Mass / Volume;
    unit kilograms_per_cubic_meter = kilograms / cubic_meters;
    dimension Momentum = Mass * Velocity;
    unit kilograms_meter_per_second = kilograms / square_meters;
    dimension MomentumDensity = Momentum / Volume;
    unit kilograms_per_meter_squared_second = kilograms / (square_meters * seconds);
    dimension Force = Mass * Velocity / Time;
    unit newtons = kilograms * meters / (seconds^2);
    dimension Pressure = Force / Area;
    unit pascals: Pressure = newtons / square_meters;
    #[base(Temperature)]
    unit kelvin: Temperature;
    dimension Energy = Force * Length;
    unit joules = newtons * meters;
    dimension SpecificEnergy = Energy / Mass;
    unit joules_per_kilogram = joules / kilograms;
    dimension SpecificHeatCapacity = SpecificEnergy / Temperature;
    unit joules_per_kilogram_kelvin = joules_per_kilogram / kelvin;
    dimension EnergyDensity = Energy / Volume;
    unit joules_per_cubic_meter = joules / cubic_meters;
    dimension Power = Energy / Time;
    unit watts = joules / seconds;
    dimension MassFlux = Mass / (Area * Time);
    unit kilograms_per_square_meter_second = kilograms / square_meters / seconds;
    dimension EnergyFlux = Power / Area;
    unit watts_per_square_meter = watts / square_meters;
    dimension DynamicViscosity = Pressure / Time;
    unit pascal_seconds = pascals * seconds;
    dimension KinematicViscosity = Area / Time;
    unit square_meters_per_second = square_meters / seconds;
);
