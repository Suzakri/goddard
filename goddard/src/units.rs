pub const G: f64 = 9.80665; // Standard gravity in m/s^2

// Pressure conversions

/// Convert pounds per square inch (psi) to pascals
/// 
/// # Parameters
/// - `psi` - The pressure in pounds per square inch
/// 
/// # Returns
/// - The pressure in pascals
pub fn psi_to_pascal(psi: f64) -> f64 {
    psi * 6894.76
}

/// Convert pascals to pounds per square inch (psi)
/// 
/// # Parameters
/// - `pa` - The pressure in pascals
/// 
/// # Returns
/// - The pressure in pounds per square inch
pub fn pascal_to_psi(pa: f64) -> f64 {
    pa / 6894.76
}

// Temperature conversions

/// Convert Celsius to Kelvin
/// 
/// # Parameters
/// - `celsius` - The temperature in Celsius
/// 
/// # Returns
/// - The temperature in Kelvin
pub fn celsius_to_kelvin(celsius: f64) -> f64 {
    celsius + 273.15
}

/// Convert Kelvin to Celsius
/// 
/// # Parameters
/// - `kelvin` - The temperature in Kelvin
/// 
/// # Returns
/// - The temperature in Celsius
pub fn kelvin_to_celsius(kelvin: f64) -> f64 {
    kelvin - 273.15
}

// Standard force conversions
// newton (n) = force
// kilonewton (kn) = 1000 newtons

/// Convert newtons to kilonewtons
/// 
/// # Parameters
/// - `n` - The force in newtons
/// 
/// # Returns
/// - The force in kilonewtons
pub fn newtons_to_kilonewtons(n: f64) -> f64 {
    n / 1000.0
}

/// Convert kilonewtons to newtons
/// 
/// # Parameters
/// - `kn` - The force in kilonewtons
/// 
/// # Returns
/// - The force in newtons
pub fn kilonewtons_to_newtons(kn: f64) -> f64 {
    kn * 1000.0
} 