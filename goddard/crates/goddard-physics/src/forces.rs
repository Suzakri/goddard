use crate::G;

/// Calculate thrust from mass flow rate and exhaust velocity
/// 
/// # Parameters
/// - `mass_flow` - Rate of propellant consumption (kg/s)
/// - `exhaust_velocity` - Speed of exhaust gases (m/s)
/// 
/// # Returns
/// - Thrust force in Newtons
pub fn thrust_from_mass_flow(mass_flow: f64, exhaust_velocity: f64) -> f64 {
    mass_flow * exhaust_velocity
}

/// Calculate exhaust velocity from specific impulse (ISP)
/// 
/// ISP is a measure of propellant efficiency - how much thrust we get per unit of propellant
/// 
/// Higher ISP means we get more thrust per kg of propellant:
/// - Solid rockets: ~250s
/// - Liquid H2/O2: ~450s
/// - Ion engines: >3000s (but very low mass flow)
/// 
/// # Parameters
/// - `isp` - Specific impulse in seconds
/// 
/// # Returns
/// - Exhaust velocity in m/s
pub fn exhaust_velocity_from_isp(isp: f64) -> f64 {
    isp * G
}

/// Calculate mass flow rate needed for desired thrust
/// 
/// Tells us how quickly we need to burn propellant to get a desired thrust.
/// 
/// # Parameters
/// - `thrust` - Desired thrust force (Newtons)
/// - `exhaust_velocity` - Speed of exhaust gases (m/s)
/// 
/// # Returns
/// - Required mass flow rate in kg/s
pub fn mass_flow_from_thrust(thrust: f64, exhaust_velocity: f64) -> f64 {
    thrust / exhaust_velocity
}

/// Calculate the thrust-to-weight ratio (TWR)
/// 
/// # Parameters
/// - `thrust` - Engine thrust (N)
/// - `mass` - Total rocket mass (kg)
/// 
/// # Returns
/// - Thrust-to-weight ratio (dimensionless)
pub fn thrust_to_weight_ratio(thrust: f64, mass: f64) -> f64 {
    thrust / (mass * G)
}