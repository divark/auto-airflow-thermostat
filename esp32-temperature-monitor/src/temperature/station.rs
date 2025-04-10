use crate::temperature::TemperatureUnit;

use embedded_hal::digital::PinState;
use esp_hal::{
    gpio::{Flex, Pull},
    time::{Duration, Instant},
};
use log::error;
use temperature_monitor_interface::{Bit, BitReader, ByteReader, Endian};

pub trait TemperatureStation {
    fn get_temperature(&mut self, temperature_unit_preference: &TemperatureUnit) -> f32;
}

/// Makes the device go to sleep (wait) for the specified
/// duration.
pub fn wait(duration: Duration) {
    let delay_start = Instant::now();
    while delay_start.elapsed() < duration {}
}

/// Returns the amount of time passed waiting for the Input pin to read back
/// either High or Low.
pub fn wait_for<'a>(input_pin: &Flex<'a>, desired_pin_state: PinState) -> Duration {
    let mut time_passed = Instant::now();
    let max_time_to_wait = Duration::from_secs(20);

    loop {
        let has_reached_pin_state = match desired_pin_state {
            PinState::High => input_pin.is_high(),
            PinState::Low => input_pin.is_low(),
        };

        if has_reached_pin_state {
            break;
        }

        if time_passed.elapsed() >= max_time_to_wait {
            error!("wait_for: Waited for longer than max time (2 seconds).");
            break;
        }
    }

    time_passed.elapsed()
}

pub struct PinBitReader<'a> {
    pin: Flex<'a>,
}

impl<'a> PinBitReader<'a> {
    pub fn new(pin: Flex<'a>) -> Self {
        let mut new_bit_reader = Self { pin };

        new_bit_reader.prepare_to_read_bits();

        new_bit_reader
    }

    /// Communicates to DHT22 sensor the necessary prerequisites to start
    /// reading in the temperature.
    fn prepare_to_read_bits(&mut self) {
        self.pin.set_low();
        wait(Duration::from_millis(18));

        self.pin.set_high();
        wait(Duration::from_micros(40));

        wait_for(&mut self.pin, PinState::High);
        wait(Duration::from_micros(80));
    }
}

impl<'a> BitReader for PinBitReader<'a> {
    fn read_next_bit(&mut self) -> Option<Bit> {
        let (zero_bit_lower_bound_time, zero_bit_upper_bound_time) =
            (Duration::from_micros(26), Duration::from_micros(28));
        let one_bit_time = Duration::from_micros(70);
        wait(Duration::from_micros(50));

        let time_passed = wait_for(&mut self.pin, PinState::Low);
        let is_zero_bit =
            time_passed >= zero_bit_lower_bound_time && time_passed <= zero_bit_upper_bound_time;
        if is_zero_bit {
            wait(one_bit_time - time_passed);
            Some(Bit::Zero)
        } else {
            Some(Bit::One)
        }
    }
}

pub struct Esp32TemperatureStation<'a> {
    byte_reader: ByteReader<PinBitReader<'a>>,
}

impl<'a> Esp32TemperatureStation<'a> {
    pub fn new(mut input_pin: Flex<'a>) -> Self {
        input_pin.enable_input(true);
        input_pin.set_as_open_drain(Pull::None);

        let pin_bit_reader = PinBitReader::new(input_pin);
        let byte_reader = ByteReader::new(pin_bit_reader);
        Self { byte_reader }
    }
}

impl<'a> TemperatureStation for Esp32TemperatureStation<'a> {
    fn get_temperature(&mut self, temperature_unit_preference: &TemperatureUnit) -> f32 {
        // In the documentation for the DHT22, the collecting period must be
        // greater than 2 seconds, so we have to wait 2 seconds each time.
        wait(Duration::from_secs(2));

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
