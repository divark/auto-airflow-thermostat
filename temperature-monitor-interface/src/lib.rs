#![no_std]

#[derive(Debug)]
pub enum TemperatureUnits {
    Celsius,
    Fahrenheit,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Temperature {
    Celsius(f32),
    Fahrenheit(f32),
}

impl Temperature {
    pub fn convert_to(&self, desired_temperature_units: TemperatureUnits) -> Temperature {
        match desired_temperature_units {
            TemperatureUnits::Celsius => match &self {
                Temperature::Celsius(celcius_temp) => Temperature::Celsius(*celcius_temp),
                Temperature::Fahrenheit(fahrenheit_temp) => {
                    let fahrenheit_to_celcius_temp = (fahrenheit_temp - 32.0) / 1.8;
                    Temperature::Celsius(fahrenheit_to_celcius_temp)
                }
            },
            TemperatureUnits::Fahrenheit => match &self {
                Temperature::Fahrenheit(fahrenheit_temp) => {
                    Temperature::Fahrenheit(*fahrenheit_temp)
                }
                Temperature::Celsius(celcius_temp) => {
                    let celcius_to_fahrenheit_temp = (celcius_temp * 1.8) + 32.0;
                    Temperature::Fahrenheit(celcius_to_fahrenheit_temp)
                }
            },
        }
    }
}

pub trait TemperatureReading {
    fn read_temperature(&mut self, desired_temperature_units: TemperatureUnits) -> Temperature;
}
