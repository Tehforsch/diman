//! Example showing gas equation of state conversions

use diman::si::f64::{
    Dimensionless, EnergyDensity, MassDensity, MomentumDensity, Pressure, SpecificEnergy,
    SpecificHeatCapacity, Temperature, Velocity,
};

#[derive(Debug)]
struct Primitive {
    pressure: Pressure,
    velocity: [Velocity; 3],
    temperature: Temperature,
}

impl Primitive {
    fn ntp() -> Self {
        Self {
            pressure: Pressure::pascals(100e3),
            velocity: [Velocity::meters_per_second(100.0); 3],
            temperature: Temperature::kelvin(293.15),
        }
    }
}

#[derive(Debug)]
struct Conservative {
    density: MassDensity,
    momentum: [MomentumDensity; 3],
    energy: EnergyDensity,
}

#[derive(Debug)]
struct IdealGas {
    #[allow(dead_code)]
    cp: SpecificHeatCapacity,
    cv: SpecificHeatCapacity,
    r: SpecificHeatCapacity,
    gamma: Dimensionless,
}

impl IdealGas {
    fn air() -> Self {
        let cp = SpecificHeatCapacity::joules_per_kilogram_kelvin(1004.);
        let cv = SpecificHeatCapacity::joules_per_kilogram_kelvin(717.);
        Self {
            cp,
            cv,
            r: cp - cv,
            gamma: cp / cv,
        }
    }
}

impl From<&Primitive> for Conservative {
    fn from(s: &Primitive) -> Self {
        let gas = IdealGas::air();
        let density: MassDensity = s.pressure / (gas.r * s.temperature);
        let energy_internal: SpecificEnergy = gas.cv * s.temperature;
        let energy_kinetic = 0.5
            * s.velocity
                .iter()
                .map(|v| v.powi::<2>())
                .sum::<SpecificEnergy>();
        Self {
            density,
            momentum: s.velocity.map(|v| density * v),
            energy: density * (energy_internal + energy_kinetic),
        }
    }
}

impl From<&Conservative> for Primitive {
    fn from(c: &Conservative) -> Self {
        let gas = IdealGas::air();
        let velocity = c.momentum.map(|m| m / c.density);
        let energy_kinetic = 0.5
            * velocity
                .iter()
                .map(|v| v.powi::<2>())
                .sum::<SpecificEnergy>();
        let e_internal = c.energy / c.density - energy_kinetic;
        Self {
            pressure: (gas.gamma - 1.0) * c.density * e_internal,
            velocity,
            temperature: e_internal / gas.cv,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primitive_to_conservative_to_primitive() {
        let s = Primitive::ntp();
        let c = Conservative::from(&s);
        let t = Primitive::from(&c);
        assert!(((s.pressure - t.pressure) / s.pressure).abs() < 1e-14);
        assert!(((s.temperature - t.temperature) / s.temperature).abs() < 1e-14);
    }
}
