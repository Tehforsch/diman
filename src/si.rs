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

    dimension Angle = 1;  // SI: plane angle
    dimension SolidAngle = Angle^2;

    dimension Area = Length^2;
    dimension Volume = Length^3;
    dimension Wavenumber = 1 / Length;

    dimension Frequency = 1 / Time;
    dimension Velocity = Length / Time;
    dimension Acceleration = Length / Time^2;
    dimension Jerk = Length / Time^3;
    dimension FlowRate = Volume / Time;

    dimension Momentum = Mass * Velocity;
    dimension Force = Mass * Acceleration;
    dimension Energy = Momentum^2 / Mass;
    dimension Power = Energy / Time;
    dimension Pressure = Force / Area;
    dimension Action = Energy * Time;
    dimension MassDensity = Mass / Length^3;
    dimension MomentOfInertia = Mass * Length^2 / Angle^2;
    dimension AngularMomentum = MomentOfInertia * Angle / Time;
    dimension Torque = Length * Force / Angle;
    dimension EnergyDensity = Energy / Volume;
    dimension MassFlow = Mass / Time;

    dimension ElectricCharge = Current * Time;
    dimension Voltage = Energy / ElectricCharge;  // ISQ: electric tension, SI: electric potential difference
    dimension Capacitance = ElectricCharge / Voltage;
    dimension ElectricResistance = Voltage / Current;
    dimension Resistivity = ElectricResistance * Length;
    dimension ElectricConductance = 1 / ElectricResistance;
    dimension Conductivity = ElectricConductance / Length;
    dimension MagneticFluxDensity = Force / (ElectricCharge * Velocity);
    dimension MagneticFlux = MagneticFluxDensity * Area;
    dimension MagneticFieldStrength = Current / Length;
    dimension Inductance = MagneticFlux / Current;
    dimension ElectricChargeDensity = ElectricCharge / Volume;
    dimension CurrentDensity = Current / Area;
    dimension ElectricDipoleMoment = ElectricCharge * Length;
    dimension ElectricQuadrupoleMoment = ElectricCharge * Length^2;
    dimension MagneticDipoleMoment = Current * Area;
    dimension ElectricFieldStrength = Voltage / Length;
    dimension ElectricDisplacementFieldStrength = ElectricCharge / Area;
    dimension ElectricPermittivity = Time^4 * Current^2 / Mass / Length^3 * Angle;
    dimension MagneticPermeability = Length * Mass / Time^2 / Current^2 / Angle;
    dimension Polarizability = ElectricDipoleMoment / ElectricFieldStrength;
    dimension ElectricMobility = Velocity / ElectricFieldStrength;

    dimension Entropy = Energy / Temperature;
    dimension HeatCapacity = Energy / Temperature;
    dimension SpecificHeatCapacity = HeatCapacity / Mass;
    dimension ThermalConductivity = Power / (Length * Temperature);
    dimension ThermalTransmittance = Power / (Length^2 * Temperature);

    dimension MolarMass = Mass / AmountOfSubstance;
    dimension MolarVolume = Volume / AmountOfSubstance;
    dimension CatalyticActivity = AmountOfSubstance / Time;
    dimension Molarity = AmountOfSubstance / Volume;
    dimension Molality = AmountOfSubstance / Mass;
    dimension ChemicalPotential = Energy / AmountOfSubstance;
    dimension MolarHeatCapacity = HeatCapacity / AmountOfSubstance;

    dimension LuminousFlux = LuminousIntensity * Angle^2;
    dimension Illuminance = LuminousFlux / Area;
    dimension Irradiance = Power / Area;

    dimension Activity = 1 / Time;
    dimension AbsorbedDose = Energy / Mass;
    dimension EquivalentDose = Energy / Mass;  // also: dose equivalent
    dimension SpecificActivity = Activity / Mass;

    dimension DynamicViscosity = Pressure * Time;

    dimension KinematicViscosity = Length^2 / Time;

    dimension MomentumDensity = Momentum / Volume;
    dimension SpecificEnergy = Energy / Mass;
    dimension MassFlux = Mass / (Area * Time);
    dimension EnergyFlux = Power / Area;

    #[metric_prefixes]
    #[symbol(m)]
    #[alias(metre, metres, meters)]
    #[base(Length)]
    unit meter: Length;

    #[metric_prefixes]
    #[symbol(s)]
    #[alias(seconds)]
    #[base(Time)]
    unit second: Time;

    #[alias(kilograms)]
    #[base(Mass)]
    #[symbol(kg)]
    unit kilogram: Mass;

    #[metric_prefixes(skip: kilo)]
    #[symbol(g)]
    #[alias(grams)]
    unit gram: Mass = 1.0e-3 * kilogram;

    #[metric_prefixes]
    #[symbol(A)]
    #[alias(amperes)]
    #[base(Current)]
    unit ampere: Current;

    #[metric_prefixes]
    #[symbol(K)]
    #[alias(kelvins)]
    #[base(Temperature)]
    unit kelvin: Temperature;

    #[metric_prefixes]
    #[symbol(mol)]
    #[alias(moles)]
    #[base(AmountOfSubstance)]
    unit mole: AmountOfSubstance;

    #[metric_prefixes]
    #[symbol(cd)]
    #[alias(candelas)]
    #[base(LuminousIntensity)]
    unit candela: LuminousIntensity;

    // derived units

    #[metric_prefixes]
    #[symbol(rad)]
    #[alias(radians)]
    unit radian: Angle = meter / meter;

    #[metric_prefixes]
    #[symbol(sr)]
    #[alias(steradians)]
    unit steradian: SolidAngle = radian^2;

    #[metric_prefixes]
    #[symbol(Hz)]
    unit hertz: Frequency = 1 / second;

    #[metric_prefixes]
    #[symbol(N)]
    #[alias(newtons)]
    unit newton: Force = kilogram meter / second^2;

    #[metric_prefixes]
    #[symbol(Pa)]
    #[alias(pascals)]
    unit pascal: Pressure = newton / meter^2;

    #[metric_prefixes]
    #[symbol(J)]
    #[alias(joules)]
    unit joule: Energy = newton meter;

    #[metric_prefixes]
    #[symbol(W)]
    #[alias(watts)]
    unit watt: Power = joule / second;

    #[metric_prefixes]
    #[symbol(C)]
    #[alias(coulombs)]
    unit coulomb: ElectricCharge = ampere second;

    #[metric_prefixes]
    #[symbol(V)]
    #[alias(volts)]
    unit volt: Voltage = kilogram meter^2 / (second^3 ampere);

    #[metric_prefixes]
    #[symbol(F)]
    #[alias(farads)]
    unit farad: Capacitance = coulomb / volt;

    #[metric_prefixes]
    #[symbol(Ω)]
    #[alias(ohms)]
    unit ohm: ElectricResistance = volt / ampere;

    #[metric_prefixes]
    #[symbol(S)]
    unit siemens: ElectricConductance = 1 / ohm;

    #[metric_prefixes]
    #[symbol(Wb)]
    #[alias(webers)]
    unit weber: MagneticFlux = volt second;

    #[metric_prefixes]
    #[symbol(T)]
    #[alias(teslas)]
    unit tesla: MagneticFluxDensity = weber / meter^2;

    #[metric_prefixes]
    #[symbol(H)]
    #[alias(henrys)]
    unit henry: Inductance = weber / ampere;

    #[metric_prefixes]
    #[symbol(lm)]
    #[alias(lumens)]
    unit lumen: LuminousFlux = candela steradian;

    #[metric_prefixes]
    #[symbol(lx)]
    unit lux: Illuminance = lumen / meter^2;

    #[metric_prefixes]
    #[symbol(Bq)]
    #[alias(becquerels)]
    unit becquerel: Activity = 1 / second;

    #[metric_prefixes]
    #[symbol(Gy)]
    #[alias(grays)]
    unit gray: AbsorbedDose = joule / kilogram;

    #[metric_prefixes]
    #[symbol(Sv)]
    #[alias(sieverts)]
    unit sievert: EquivalentDose = joule / kilogram;

    #[metric_prefixes]
    #[symbol(kat)]
    #[alias(katals)]
    unit katal: CatalyticActivity = mole / second;

    // SI accepted units
    #[symbol(min)]
    #[alias(minutes)]
    unit minute: Time = 60 second;

    #[symbol(h)]
    #[alias(hours)]
    unit hour: Time = 60 minute;

    #[symbol(day)]
    #[alias(days)]
    unit day: Time = 24 hour;

    #[symbol(au)]
    #[alias(astronomicalunits)]
    unit astronomicalunit: Length = 149_597_870_700 meter;

    constant PI = 3.141592653589793;
    //TODO(minor): Support using ° here.
    #[symbol(deg)]
    #[alias(degrees)]
    unit degree: Angle = PI / 180 * radian;

    #[alias(arcminutes)]
    unit arcminute: Angle = 1 / 60 * degree;

    #[alias(arcseconds)]
    unit arcsecond: Angle = 1 / 60 * arcminute;

    #[alias(ares)]
    unit are: Area = 100 meter^2;

    #[symbol(ha)]
    #[alias(hectares)]
    unit hectare: Area = 100 are;

    #[metric_prefixes]
    #[symbol(l)]
    #[alias(litres)]
    unit litre: Volume = decimeter^3;

    #[metric_prefixes]
    #[alias(tonnes)]
    unit tonne: Mass = 10^3 kilogram;

    #[symbol(Da)]
    #[alias(daltons)]
    unit dalton: Mass = 1.660_539_066_60e-27 kilogram;

    #[metric_prefixes]
    #[symbol(eV)]
    #[alias(electronvolts)]
    unit electronvolt: Energy = 1.602_176_634e-19 joule;

    #[metric_prefixes]
    #[alias(joules_per_kilogram)]
    unit joule_per_kilogram: SpecificEnergy = joule / kilogram;

    #[metric_prefixes]
    #[alias(joules_per_kilogram_kelvin)]
    unit joule_per_kilogram_kelvin: SpecificHeatCapacity  = joule_per_kilogram / kelvin;

    #[metric_prefixes]
    #[alias(meters_per_second)]
    unit meter_per_second: Velocity = meter / second;

    #[alias(square_metre, square_meters, square_metres)]
    unit square_meter: Area = meter^2;

    #[alias(cubic_metre, cubic_meters, cubic_metres)]
    unit cubic_meter: Volume = meter^3;
);
