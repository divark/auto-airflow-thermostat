use crate::temperature::TemperatureUnit;

use esp_hal::gpio::Pin;
use temperature_monitor_interface::{Bit, BitReader, ByteReader};

pub trait TemperatureStation {
    fn get_temperature(&self, temperature_unit_preference: TemperatureUnit) -> usize;
}

pub struct PinBitReader<U>
where
    U: Pin,
{
    pin: U,
}

impl<U> PinBitReader<U>
where
    U: Pin,
{
    pub fn new(pin: U) -> Self {
        Self { pin }
    }
}

impl<U> BitReader for PinBitReader<U>
where
    U: Pin,
{
    fn read_next_bit(&mut self) -> Option<Bit> {
        None
    }
}

pub struct Esp32TemperatureStation<U>
where
    U: Pin,
{
    byte_reader: ByteReader<PinBitReader<U>>,
}

impl<U> Esp32TemperatureStation<U>
where
    U: Pin,
{
    pub fn new(pin: U) -> Self {
        let pin_bit_reader = PinBitReader::new(pin);
        Self {
            byte_reader: ByteReader::new(pin_bit_reader),
        }
    }
}
