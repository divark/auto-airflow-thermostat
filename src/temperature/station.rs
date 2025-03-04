use crate::temperature::TemperatureUnit;

pub trait TemperatureStation {
    fn get_temperature(&self, temperature_unit_preference: TemperatureUnit) -> usize;
}

pub struct Esp32TemperatureStation {}
