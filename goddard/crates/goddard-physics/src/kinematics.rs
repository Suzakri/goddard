use goddard_core::G;

/// Calculate the ideal delta-v (change in velocity) using the Tsiolkovsky rocket equation
/// 
/// # Parameters
/// - `exhaust_velocity` - The exhaust velocity in meters per second
/// - `initial_mass` - The initial mass in kilograms
/// - `final_mass` - The final mass in kilograms
/// 
/// # Returns
/// - The ideal delta-v in meters per second
pub fn ideal_delta_v(exhaust_velocity: f64, initial_mass: f64, final_mass: f64) -> f64 {
    exhaust_velocity * (initial_mass / final_mass).ln()
}

/// Calculate the burn time to achieve a given delta-v
/// 
/// # Parameters
/// - `initial_mass` - The initial mass in kilograms
/// - `mass_flow_rate` - The mass flow rate in kilograms per second
/// - `delta_v` - The desired change in velocity in meters per second
/// - `exhaust_velocity` - The exhaust velocity in meters per second
/// 
/// # Returns
/// - The burn time in seconds
pub fn burn_time(initial_mass: f64, mass_flow_rate: f64, delta_v: f64, exhaust_velocity: f64) -> f64 {
    let mass_ratio = (-delta_v / exhaust_velocity).exp();
    (initial_mass / mass_flow_rate.abs()) * (1.0 - mass_ratio)
}

/// Calculate the maximum altitude for a vertical launch, neglecting air resistance
/// 
/// # Parameters
/// - `initial_velocity` - The initial velocity in meters per second
/// 
/// # Returns
/// - The maximum altitude in meters
pub fn max_altitude_no_drag(initial_velocity: f64) -> f64 {
    initial_velocity * initial_velocity / (2.0 * G)
}

/// Calculate time to apogee (highest point) for a vertical launch, neglecting air resistance
/// 
/// # Parameters
/// - `initial_velocity` - The initial velocity in meters per second
/// 
/// # Returns
/// - The time to apogee in seconds
pub fn time_to_apogee_no_drag(initial_velocity: f64) -> f64 {
    initial_velocity / G
} 