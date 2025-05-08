#![no_std]
#![no_main]

use core::panic::PanicInfo;

use esp32_temperature_monitor::Dht11TemperatureReader;
use temperature_monitor_interface::{TemperatureReading, TemperatureUnits};

use esp_hal::main;
use esp_println::println;

#[panic_handler]
fn on_panic(info: &PanicInfo) -> ! {
    println!("Panicked! Info: {:?}", info);
    loop {}
}

#[main]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();

    let mut temperature_reader = Dht11TemperatureReader::new();
    let desired_temperature_units = TemperatureUnits::Fahrenheit;
    loop {
        let read_temperature = temperature_reader.read_temperature(desired_temperature_units);

        println!("Read temperature: {} F", read_temperature.get_reading());
    }
}
