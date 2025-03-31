use crate::temperature::TemperatureUnit;

use esp_hal::gpio::InputPin;
use temperature_monitor_interface::{Bit, BitReader, ByteReader, Endian};

pub trait TemperatureStation {
    fn get_temperature(&mut self, temperature_unit_preference: &TemperatureUnit) -> f32;
}

pub struct PinBitReader<U>
where
    U: embedded_hal::digital::InputPin,
{
    pin: U,
}

impl<U> PinBitReader<U>
where
    U: embedded_hal::digital::InputPin,
{
    pub fn new(pin: U) -> Self {
        Self { pin }
    }
}

impl<U> BitReader for PinBitReader<U>
where
    U: embedded_hal::digital::InputPin,
{
    fn read_next_bit(&mut self) -> Option<Bit> {
        self.pin.is_high();
        None
    }
}

pub struct Esp32TemperatureStation<U>
where
    U: embedded_hal::digital::InputPin,
{
    byte_reader: ByteReader<PinBitReader<U>>,
}

impl<U> Esp32TemperatureStation<U>
where
    U: embedded_hal::digital::InputPin,
{
    pub fn new(pin: U) -> Self {
        let pin_bit_reader = PinBitReader::new(pin);
        Self {
            byte_reader: ByteReader::new(pin_bit_reader),
        }
    }
}

impl<U> TemperatureStation for Esp32TemperatureStation<U>
where
    U: embedded_hal::digital::InputPin,
{
    fn get_temperature(&mut self, temperature_unit_preference: &TemperatureUnit) -> f32 {
        let bit_reading_order = Endian::Big;

        let temperature_int = self.byte_reader.read(&bit_reading_order) as f32;
        let temperature_decimal = self.byte_reader.read(&bit_reading_order) as f32 / 100.0;
        let read_temperature = temperature_int + temperature_decimal;

        let humidity_int = self.byte_reader.read(&bit_reading_order) as f32;
        let humidity_decimal = self.byte_reader.read(&bit_reading_order) as f32 / 100.0;
        let _humidity = humidity_int + humidity_decimal;

        let _checksum = self.byte_reader.read(&bit_reading_order);

        read_temperature
    }
}
