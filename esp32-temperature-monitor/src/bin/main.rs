#![no_std]
#![no_main]

use esp32_temperature_monitor::temperature::station::{
    Esp32TemperatureStation, TemperatureStation,
};
use esp32_temperature_monitor::temperature::TemperatureUnit;
use esp_hal::clock::CpuClock;
use esp_hal::gpio::Flex;
use esp_hal::main;
use esp_hal::time::{Duration, Instant};
use esp_hal::timer::timg::TimerGroup;
use log::info;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

extern crate alloc;

#[main]
fn main() -> ! {
    // generator version: 0.3.0

    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(size: 72 * 1024);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let _init = esp_wifi::init(
        timg0.timer0,
        esp_hal::rng::Rng::new(peripherals.RNG),
        peripherals.RADIO_CLK,
    )
    .unwrap();

    let temperature_reading_pin = peripherals.GPIO0;
    let pin_communicator = Flex::new(temperature_reading_pin);
    let mut temperature_monitor = Esp32TemperatureStation::new(pin_communicator);

    let temperature_units = TemperatureUnit::Fahrenheit;

    loop {
        let temperature = temperature_monitor.get_temperature(&temperature_units);
        info!("Temperature: {:.1}F", temperature);

        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(500) {}
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-beta.0/examples/src/bin
}
