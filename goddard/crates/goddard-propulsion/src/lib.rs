use goddard_physics::forces;

pub struct Engine {
    // Thrust - Newtons
    thrust: f64,
    // Specific Impulse - Seconds
    isp: f64,
    // Mass Flow Rate - kg/s
    mass_flow: f64,
}

impl Engine {
    pub fn new(thrust: f64, isp: f64, mass_flow: f64) -> Self {
        Engine { thrust, isp, mass_flow }
    }

    pub fn thrust(&self) -> f64 { self.thrust }
    pub fn isp(&self) -> f64 { self.isp }
    pub fn mass_flow_rate(&self) -> f64 { self.mass_flow }
    pub fn exhaust_velocity(&self) -> f64 { 
        forces::exhaust_velocity_from_isp(self.isp)
    }
} 