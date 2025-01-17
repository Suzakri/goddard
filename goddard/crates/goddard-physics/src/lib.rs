pub mod forces;
pub mod kinematics;
pub mod units;

// Re-export commonly used items
pub use forces::*;
pub use kinematics::*;
pub use units::*;

// Physical constants
pub const G: f64 = 9.80665;    // Standard gravitational acceleration (m/s²)
pub const P_ATM: f64 = 101325.0; // Standard atmospheric pressure at sea level (Pa)
pub const R: f64 = 8.31446261815324; // Universal gas constant (J/(mol·K))

/// Gas state at a point
#[derive(Debug, Clone, Copy)]
pub struct GasState {
    pub pressure: f64,    // Pressure in Pa
    pub temperature: f64, // Temperature in K
    pub density: f64,     // Density in kg/m³
    pub gamma: f64,       // Specific heat ratio (cp/cv)
}

/// Flow conditions at a point
#[derive(Debug, Clone, Copy)]
pub struct FlowState {
    pub gas: GasState,
    pub mach: f64,        // Mach number
    pub velocity: f64,    // Velocity in m/s
    pub mass_flow: f64,   // Mass flow rate in kg/s
    pub area: f64,        // Cross-sectional area in m²
} 