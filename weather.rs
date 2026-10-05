use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Weather {
    pub city: String,
    pub temperature: f64,
    pub feels_like: f64,
    pub humidity: u32,
    pub wind_speed: f64,
    pub condition: String,
}


// ========================================
// DISPLAY WEATHER
// ========================================

impl Weather {

    pub fn display(&self) {

        println!();
        println!("========================================");
        println!(
            "          WEATHER — {}",
            self.city.to_uppercase()
        );
        println!("========================================");

        println!(
            "Temperature: {:.1}°C",
            self.temperature
        );

        println!(
            "Feels like: {:.1}°C",
            self.feels_like
        );

        println!(
            "Humidity: {}%",
            self.humidity
        );

        println!(
            "Wind speed: {:.1} km/h",
            self.wind_speed
        );

        println!(
            "Condition: {}",
            self.condition
        );

        println!("========================================");
    }
}