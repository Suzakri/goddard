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

    /// Convert bar to pascals (Pa)
    pub fn bar_to_pascal(bar: f64) -> f64 {
        bar * 1e5
    }

    /// Convert pascals (Pa) to bar
    pub fn pascal_to_bar(pa: f64) -> f64 {
        pa / 1e5
    }

    /// Convert atmospheres (atm) to pascals (Pa)
    pub fn atm_to_pascal(atm: f64) -> f64 {
        atm * 101325.0
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

    /// Convert Kelvin to Fahrenheit
    pub fn kelvin_to_fahrenheit(kelvin: f64) -> f64 {
        (kelvin - 273.15) * 9.0/5.0 + 32.0
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

    /// Convert pound-force to newtons
    pub fn lbf_to_newtons(lbf: f64) -> f64 {
        lbf * 4.448222
    }

    /// Convert newtons to pound-force
    pub fn newtons_to_lbf(n: f64) -> f64 {
        n / 4.448222
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

    /// Convert kilograms to slugs
    pub fn kg_to_slug(kg: f64) -> f64 {
        kg * 0.06852177
    }

    /// Convert slugs to kilograms
    pub fn slug_to_kg(slug: f64) -> f64 {
        slug / 0.06852177
    }
}

/// Length conversions
pub mod length {
    /// Convert meters to feet
    pub fn meters_to_feet(m: f64) -> f64 {
        m * 3.28084
    }

    /// Convert feet to meters
    pub fn feet_to_meters(ft: f64) -> f64 {
        ft / 3.28084
    }

    /// Convert inches to meters
    pub fn inches_to_meters(inch: f64) -> f64 {
        inch * 0.0254
    }

    /// Convert meters to inches
    pub fn meters_to_inches(m: f64) -> f64 {
        m / 0.0254
    }
} 