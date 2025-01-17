pub const G: f64 = 9.80665; // Standard gravity in m/s^2

// Pressure conversions
// psi = pounds per square inch
// pascal (pa) = force per unit area
pub fn psi_to_pascal(psi: f64) -> f64 {
    psi * 6894.76
}

pub fn pascal_to_psi(pa: f64) -> f64 {
    pa / 6894.76
}

// Temperature conversions
pub fn celsius_to_kelvin(celsius: f64) -> f64 {
    celsius + 273.15
}

pub fn kelvin_to_celsius(kelvin: f64) -> f64 {
    kelvin - 273.15
}

// Standard force conversions
// newton (n) = force
// kilonewton (kn) = 1000 newtons
pub fn newtons_to_kilonewtons(n: f64) -> f64 {
    n / 1000.0
}

pub fn kilonewtons_to_newtons(kn: f64) -> f64 {
    kn * 1000.0
} 