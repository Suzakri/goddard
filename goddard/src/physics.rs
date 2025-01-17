use crate::units::G;

/// Calculate exhaust velocity from specific impulse (ISP)
/// 
/// Represents the speed at which propellant is ejected from the engine
/// 
/// # Parameters
/// - `isp` - The specific impulse in seconds
/// 
/// # Returns
/// - The exhaust velocity in meters per second
pub fn exhaust_velocity_from_isp(isp: f64) -> f64 {
    isp * G
}

/// Calculate thrust from mass flow rate and exhaust velocity
/// 
/// Represents the force exerted by the engine on the rocket
/// 
/// # Parameters
/// - `mass_flow` - The mass flow rate in kilograms per second
/// - `exhaust_velocity` - The exhaust velocity in meters per second
/// 
/// # Returns
/// - The thrust in newtons
pub fn thrust_from_mass_flow(mass_flow: f64, exhaust_velocity: f64) -> f64 {
    mass_flow * exhaust_velocity
}

/// Calculate specific impulse (ISP) from exhaust velocity
///
/// Represents how many seconds of thrust a unit of propellant can produce
/// 
/// # Parameters
/// - `exhaust_velocity` - The exhaust velocity in meters per second
/// 
/// # Returns
/// - The specific impulse (ISP) in seconds
pub fn isp_from_exhaust_velocity(exhaust_velocity: f64) -> f64 {
    exhaust_velocity / G
}

/// Calculate mass flow rate from thrust and exhaust velocity
/// 
/// Represents the rate at which propellant is consumed by the engine
/// 
/// # Parameters
/// - `thrust` - The thrust in newtons
/// - `exhaust_velocity` - The exhaust velocity in meters per second
/// 
/// # Returns
/// - The mass flow rate in kilograms per second
pub fn mass_flow_from_thrust(thrust: f64, exhaust_velocity: f64) -> f64 {
    thrust / exhaust_velocity
}

/// Calculate the thrust-to-weight ratio
/// 
/// Represents the ratio of thrust to the weight of the rocket
/// 
/// # Parameters
/// - `thrust` - The thrust in newtons
/// - `mass` - The mass of the rocket in kilograms
/// 
/// # Returns
/// - The ratio of thrust to the weight of the rocket
pub fn thrust_to_weight_ratio(thrust: f64, mass: f64) -> f64 {
    thrust / (mass * G)
}