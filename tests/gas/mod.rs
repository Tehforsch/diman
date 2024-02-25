//! Example showing gas equation of state conversions

use diman::si::{
    dimensions::{
        Dimensionless, EnergyDensity, MassDensity, MomentumDensity, Pressure, SpecificEnergy,
        SpecificHeatCapacity, Temperature, Velocity,
    },
    units::{joules_per_kilogram_kelvin, kelvin, meters_per_second, pascals},
};

#[derive(Debug)]
struct Primitive {
    pressure: Pressure<f64>,
    velocity: [Velocity<f64>; 3],
    temperature: Temperature<f64>,
}

impl Primitive {
    fn ntp() -> Self {
        Self {
            pressure: pascals.new(100e3),
            velocity: [meters_per_second.new(100.0); 3],
            temperature: kelvin.new(293.15),
        }
    }
}

#[derive(Debug)]
struct Conservative {
    density: MassDensity<f64>,
    momentum: [MomentumDensity<f64>; 3],
    energy: EnergyDensity<f64>,
}

#[derive(Debug)]
struct IdealGas {
    #[allow(dead_code)]
    cp: SpecificHeatCapacity<f64>,
    cv: SpecificHeatCapacity<f64>,
    r: SpecificHeatCapacity<f64>,
    gamma: Dimensionless<f64>,
}

impl IdealGas {
    fn air() -> Self {
        let cp = joules_per_kilogram_kelvin.new(1004.);
        let cv = joules_per_kilogram_kelvin.new(717.);
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
        let density: MassDensity<f64> = s.pressure / (gas.r * s.temperature);
        let energy_internal: SpecificEnergy<f64> = gas.cv * s.temperature;
        let energy_kinetic = 0.5
            * s.velocity
                .iter()
                .map(|v| v.powi::<2>())
                .sum::<SpecificEnergy<f64>>();
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
                .sum::<SpecificEnergy<f64>>();
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
