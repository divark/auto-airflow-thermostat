#![no_std]
#![no_main]

use embedded_dht_rs::dht11::Dht11;
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::time::Duration;
use esp_hal::{
    delay::Delay,
    gpio::{Level, OutputOpenDrain, Pull},
    main,
    peripherals::Peripherals,
};
use esp_println::println;

#[main]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let delay = Delay::new();

    let od_for_dht11 = OutputOpenDrain::new(peripherals.GPIO8, Level::High, Pull::None);

    let mut dht11 = Dht11::new(od_for_dht11, delay);

    loop {
        delay.delay(Duration::millis(5000));

        match dht11.read() {
            Ok(sensor_reading) => println!(
                "DHT 11 Sensor - Temperature: {} °C, humidity: {} %",
                sensor_reading.temperature, sensor_reading.humidity
            ),
            Err(error) => println!("An error occurred while trying to read sensor: {:?}", error),
        }

        println!("-----");
    }
}
