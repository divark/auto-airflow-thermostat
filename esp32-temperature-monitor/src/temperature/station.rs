use crate::temperature::TemperatureUnit;

use embedded_hal::digital::PinState;
use esp_hal::{
    gpio::InputPin,
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
pub fn wait_for<'a, U>(input_pin: &'a mut U, desired_pin_state: PinState) -> Duration
where
    U: embedded_hal::digital::InputPin + embedded_hal::digital::OutputPin,
{
    let mut time_passed = Instant::now();
    let max_time_to_wait = Duration::from_secs(2);

    let mut num_tries = 0;
    let num_tries_max = 10;
    loop {
        let pin_state_status = match desired_pin_state {
            PinState::High => input_pin.is_high(),
            PinState::Low => input_pin.is_low(),
        };

        if let Ok(has_reached_pin_state) = pin_state_status {
            if has_reached_pin_state {
                break;
            }
        } else if let Err(error_msg) = pin_state_status {
            num_tries += 1;

            if num_tries == num_tries_max {
                error!("wait_for: Number of tries exceeded.");
                break;
            }
        }

        wait(Duration::from_micros(1));

        if time_passed.elapsed() >= max_time_to_wait {
            error!("wait_for: Waited for longer than max time (2 seconds).");
            break;
        }
    }

    time_passed.elapsed()
}

pub struct PinBitReader<'a, U>
where
    U: embedded_hal::digital::InputPin,
{
    pin: &'a mut U,
}

impl<'a, U> PinBitReader<'a, U>
where
    U: embedded_hal::digital::InputPin,
{
    pub fn new(pin: &'a mut U) -> Self {
        Self { pin }
    }
}

impl<'a, U> BitReader for PinBitReader<'_, U>
where
    U: embedded_hal::digital::InputPin + embedded_hal::digital::OutputPin,
{
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

pub struct Esp32TemperatureStation<U>
where
    U: embedded_hal::digital::InputPin,
{
    input_pin: U,
}

impl<U> Esp32TemperatureStation<U>
where
    U: embedded_hal::digital::InputPin + embedded_hal::digital::OutputPin,
{
    pub fn new(input_pin: U) -> Self {
        Self { input_pin }
    }

    /// Communicates to DHT22 sensor the necessary prerequisites to start
    /// reading in the temperature.
    fn prepare_to_read_temperature(&mut self) {
        // In the documentation for the DHT22, the collecting period must be
        // greater than 2 seconds, so we have to wait 2 seconds each time.
        wait(Duration::from_secs(2));

        self.input_pin.set_low();
        wait(Duration::from_millis(1));

        self.input_pin.set_high();
        wait(Duration::from_micros(40));

        wait_for(&mut self.input_pin, PinState::High);
        wait(Duration::from_micros(80));
    }
}

impl<U> TemperatureStation for Esp32TemperatureStation<U>
where
    U: embedded_hal::digital::InputPin + embedded_hal::digital::OutputPin,
{
    fn get_temperature(&mut self, temperature_unit_preference: &TemperatureUnit) -> f32 {
        self.prepare_to_read_temperature();

        let bit_reading_order = Endian::Big;
        let mut bit_reader = PinBitReader::new(&mut self.input_pin);
        let mut byte_reader = ByteReader::new(bit_reader);

        let temperature_int = byte_reader.read(&bit_reading_order) as f32;
        let temperature_decimal = byte_reader.read(&bit_reading_order) as f32 / 100.0;
        let read_temperature = temperature_int + temperature_decimal;

        let humidity_int = byte_reader.read(&bit_reading_order) as f32;
        let humidity_decimal = byte_reader.read(&bit_reading_order) as f32 / 100.0;
        let _humidity = humidity_int + humidity_decimal;

        let _checksum = byte_reader.read(&bit_reading_order);

        read_temperature
    }
}
