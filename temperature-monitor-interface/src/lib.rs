#![no_std]

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TemperatureUnits {
    Celsius,
    Fahrenheit,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Temperature {
    units: TemperatureUnits,
    reading: f32,
}

impl Temperature {
    pub fn new(units: TemperatureUnits, reading: f32) -> Self {
        Self { units, reading }
    }

    pub fn get_reading(&self) -> f32 {
        self.reading
    }

    pub fn get_units(&self) -> &TemperatureUnits {
        &self.units
    }

    pub fn convert_to(&self, desired_temperature_units: TemperatureUnits) -> Temperature {
        match desired_temperature_units {
            TemperatureUnits::Celsius => match self.get_units() {
                TemperatureUnits::Celsius => {
                    Temperature::new(TemperatureUnits::Celsius, self.reading)
                }
                TemperatureUnits::Fahrenheit => {
                    let fahrenheit_to_celcius_temp = (self.get_reading() - 32.0) / 1.8;
                    Temperature::new(TemperatureUnits::Celsius, fahrenheit_to_celcius_temp)
                }
            },
            TemperatureUnits::Fahrenheit => match self.get_units() {
                TemperatureUnits::Fahrenheit => {
                    Temperature::new(TemperatureUnits::Fahrenheit, self.reading)
                }
                TemperatureUnits::Celsius => {
                    let celcius_to_fahrenheit_temp = (self.get_reading() * 1.8) + 32.0;
                    Temperature::new(TemperatureUnits::Fahrenheit, celcius_to_fahrenheit_temp)
                }
            },
        }
    }
}

pub trait TemperatureReading {
    fn read_temperature(&mut self, desired_temperature_units: TemperatureUnits) -> Temperature;
}
