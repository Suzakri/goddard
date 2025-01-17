/// Pressure conversions
pub mod pressure {
    /// Convert pounds per square inch (psi) to pascals (Pa)
    pub fn psi_to_pascal(psi: f64) -> f64 {
        psi * 6894.76
    }

    /// Convert pascals (Pa) to pounds per square inch (psi)
    pub fn pascal_to_psi(pa: f64) -> f64 {
        pa / 6894.76
    }
}

/// Temperature conversions
pub mod temperature {
    /// Convert Celsius to Kelvin
    pub fn celsius_to_kelvin(celsius: f64) -> f64 {
        celsius + 273.15
    }

    /// Convert Kelvin to Celsius
    pub fn kelvin_to_celsius(kelvin: f64) -> f64 {
        kelvin - 273.15
    }

    /// Convert Fahrenheit to Kelvin
    pub fn fahrenheit_to_kelvin(fahrenheit: f64) -> f64 {
        (fahrenheit - 32.0) * 5.0/9.0 + 273.15
    }
}

/// Force conversions
pub mod force {
    /// Convert newtons to kilonewtons
    pub fn newtons_to_kilonewtons(n: f64) -> f64 {
        n / 1000.0
    }

    /// Convert kilonewtons to newtons
    pub fn kilonewtons_to_newtons(kn: f64) -> f64 {
        kn * 1000.0
    }
}

/// Mass conversions
pub mod mass {
    /// Convert kilograms to pounds
    pub fn kg_to_lbs(kg: f64) -> f64 {
        kg * 2.20462
    }

    /// Convert pounds to kilograms
    pub fn lbs_to_kg(lbs: f64) -> f64 {
        lbs / 2.20462
    }
} 