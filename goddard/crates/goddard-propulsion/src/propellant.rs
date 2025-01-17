#[derive(Clone, Copy)]
pub struct PropellantProperties {
    molecular_mass: f64,   // kg/mol
    density: f64,          // kg/m³
    temperature: f64,      // K (storage temperature)
}

// Standard properties for a few common propellants
// TODO: Add more propellants
pub mod properties {
    use super::PropellantProperties;

    pub const LOX: PropellantProperties = PropellantProperties {
        molecular_mass: 0.032,
        density: 1141.0,
        temperature: 90.0,
    };

    pub const LH2: PropellantProperties = PropellantProperties {
        molecular_mass: 0.002,
        density: 71.0,
        temperature: 20.0,
    };

    pub const RP1: PropellantProperties = PropellantProperties {
        molecular_mass: 0.175,
        density: 820.0,
        temperature: 300.0,
    };

    pub const METHANE: PropellantProperties = PropellantProperties {
        molecular_mass: 0.016,
        density: 422.6,
        temperature: 111.7,
    };
}

/// Common oxidizers used in rocket engines
pub enum Oxidizer {
    LiquidOxygen(PropellantProperties),    // LOX
    NitrogenTetroxide(PropellantProperties), // N2O4
    HydrogenPeroxide(PropellantProperties),  // H2O2
}

/// Common fuels used in rocket engines
pub enum Fuel {
    LiquidHydrogen(PropellantProperties),   // LH2
    RP1(PropellantProperties),              // Refined kerosene
    Methane(PropellantProperties),          // CH4
}

/// A propellant combination (oxidizer + fuel)
pub struct PropellantPair {
    oxidizer: Oxidizer,
    fuel: Fuel,
    optimal_mixture_ratio: f64,  // Optimal O/F ratio
    combustion_temp: f64,        // K
}

impl PropellantPair {
    pub fn lox_lh2() -> Self {
        PropellantPair {
            oxidizer: Oxidizer::LiquidOxygen(properties::LOX),
            fuel: Fuel::LiquidHydrogen(properties::LH2),
            optimal_mixture_ratio: 5.5,   // Typical O/F ratio for LOX/LH2
            combustion_temp: 3300.0,      // K
        }
    }

    pub fn lox_rp1() -> Self {
        PropellantPair {
            oxidizer: Oxidizer::LiquidOxygen(properties::LOX),
            fuel: Fuel::RP1(properties::RP1),
            optimal_mixture_ratio: 2.27,  // Typical O/F ratio for LOX/RP-1
            combustion_temp: 3570.0,      // K
        }
    }
} 