use goddard_physics::forces;

/// Combustion chamber where propellants mix and burn
pub struct CombustionChamber {
    pressure: f64,    // Chamber pressure (Pa)
    temperature: f64, // Combustion temperature (K)
    volume: f64,      // Chamber volume (m³)
}

impl CombustionChamber {
    pub fn new(pressure: f64, temperature: f64, volume: f64) -> Self {
        CombustionChamber { pressure, temperature, volume }
    }
}

/// Nozzle that accelerates exhaust gases
pub struct Nozzle {
    throat_area: f64,     // Area at narrowest point (m²)
    exit_area: f64,       // Area at nozzle exit (m²)
    expansion_ratio: f64, // exit_area / throat_area
}

impl Nozzle {
    pub fn new(throat_area: f64, exit_area: f64) -> Self {
        Nozzle {
            throat_area,
            exit_area,
            expansion_ratio: exit_area / throat_area,
        }
    }
}

/// Propellant feed system
pub struct FeedSystem {
    mass_flow_rate: f64,  // kg/s
    mixture_ratio: f64,   // oxidizer/fuel ratio
}

impl FeedSystem {
    pub fn new(mass_flow_rate: f64, mixture_ratio: f64) -> Self {
        FeedSystem { mass_flow_rate, mixture_ratio }
    }
}

pub struct Engine {
    chamber: CombustionChamber,
    nozzle: Nozzle,
    feed_system: FeedSystem,
}

impl Engine {
    pub fn new(chamber: CombustionChamber, nozzle: Nozzle, feed_system: FeedSystem) -> Self {
        Engine { chamber, nozzle, feed_system }
    }

    /// Calculate engine's thrust based on chamber conditions and nozzle
    pub fn thrust(&self) -> f64 {
        let isp = self.isp();
        let exhaust_velocity = forces::exhaust_velocity_from_isp(isp);
        forces::thrust_from_mass_flow(self.feed_system.mass_flow_rate, exhaust_velocity)
    }

    /// Estimate ISP based on chamber temperature and expansion ratio
    pub fn isp(&self) -> f64 {
        // Simplified ISP calculation based on chamber temperature and nozzle expansion
        // In reality, this depends on many more factors
        let base_isp = (self.chamber.temperature / 3000.0) * 300.0; // Temperature factor
        let nozzle_efficiency = (self.nozzle.expansion_ratio / 10.0).sqrt(); // Nozzle factor
        base_isp * nozzle_efficiency
    }

    pub fn mass_flow_rate(&self) -> f64 {
        self.feed_system.mass_flow_rate
    }

    pub fn exhaust_velocity(&self) -> f64 {
        forces::exhaust_velocity_from_isp(self.isp())
    }
} 