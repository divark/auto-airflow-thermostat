#![no_std]

use embedded_dht_rs::dht11::Dht11;

use embedded_hal::digital::{InputPin, OutputPin};

use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::gpio::{Level, OutputOpenDrain, Pull};

use temperature_monitor_interface::{Temperature, TemperatureReading, TemperatureUnits};

pub struct Dht11TemperatureReader<'a> {
    dht11_reader: Dht11<OutputOpenDrain<'a>, Delay>,
}

impl Dht11TemperatureReader<'_> {
    pub fn new() -> Self {
        let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
        let peripherals = esp_hal::init(config);

        let od_for_dht11 = OutputOpenDrain::new(peripherals.GPIO0, Level::High, Pull::None);
        let delay = Delay::new();

        let dht11_reader = Dht11::new(od_for_dht11, delay);

        Self { dht11_reader }
    }
}

impl TemperatureReading for Dht11TemperatureReader<'_> {
    fn read_temperature(&mut self, desired_temperature_units: TemperatureUnits) -> Temperature {
        let dht_reading = self.dht11_reader.read();

        let read_temperature = match dht_reading {
            Ok(sensor_reading) => {
                let sensor_temperature_celsius = sensor_reading.temperature as f32;
                Temperature::new(TemperatureUnits::Celsius, sensor_temperature_celsius)
            }
            Err(sensor_error) => panic!(
                "An error has occurred while trying to read from the dht11 sensor: {:?}",
                sensor_error
            ),
        };

        read_temperature.convert_to(desired_temperature_units)
    }
}
