/// Basic properties of a fluid at a point
#[derive(Debug, Clone, Copy)]
pub struct FluidProperties {
    density: f64,      // kg/m³ - Mass per unit volume of fluid
    velocity: f64,     // m/s - Speed of fluid
    pressure: f64,     // Pressure of fluid
    viscosity: f64,    // Viscosity/thickness of fluid
}

impl FluidProperties {
    /// Calculate Reynolds number given a characteristic length (pipe diameter, etc.)
    /// 
    /// Re = (ρvL)/μ
    /// Where:
    /// - ρ = density
    /// - v = velocity
    /// - L = characteristic length
    /// - μ = dynamic viscosity
    /// 
    /// Low Re indicates laminar (slow, predictable) flow, high Re indicates turbulent (fast, chaotic) flow
    pub fn reynolds_number(&self, characteristic_length: f64) -> f64 {
        (self.density * self.velocity * characteristic_length) / self.viscosity
    }
}

/// Calculate pressure change due to elevation (hydrostatic pressure)
/// 
/// Δp = -ρgh
/// Where:
/// - ρ = Fluid density
/// - g = Gravity
/// - h = Height difference
pub fn hydrostatic_pressure_delta(density: f64, gravity: f64, height_difference: f64) -> f64 {
    -density * gravity * height_difference
}