use std::fs;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Config {
    cash_buffer_ratio: f64,
    netto_monthly_withdrawal: f64,
    monthly_saving: f64,
    interest_rate_working: f64,
    initial_savings: f64,
    working_years: u32,
    pension_years: u32,
}

impl Config {
    fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        match Config::from_file("../../config.toml") {
            Ok(config) => println!("{:?}", config),
            Err(e) => eprintln!("Failed to read config: {}", e),
        }
    }
}