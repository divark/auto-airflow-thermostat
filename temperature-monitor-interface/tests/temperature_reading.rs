use cucumber::{World, given, then, when};

use temperature_monitor_interface::{Temperature, TemperatureReading, TemperatureUnits};

fn round_float_to_two_decimal_places(number: f32) -> f32 {
    (number * 100.0).round() / 100.0
}

fn round_to_two_decimal_places(temperature: Temperature) -> Temperature {
    match temperature {
        Temperature::Celsius(celsius_temperature) => {
            let rounded_celsius_temp = round_float_to_two_decimal_places(celsius_temperature);
            Temperature::Celsius(rounded_celsius_temp)
        }
        Temperature::Fahrenheit(fahrenheit_temperature) => {
            let rounded_fahrenheit_temp = round_float_to_two_decimal_places(fahrenheit_temperature);
            Temperature::Fahrenheit(rounded_fahrenheit_temp)
        }
    }
}

#[derive(Debug)]
struct TestTemperatureReader {
    outside_temperature: Temperature,
}

impl TestTemperatureReader {
    pub fn new(outside_temperature: Temperature) -> Self {
        Self {
            outside_temperature,
        }
    }
}

impl TemperatureReading for TestTemperatureReader {
    fn read_temperature(&mut self, desired_temperature_units: TemperatureUnits) -> Temperature {
        let temperature_read = self
            .outside_temperature
            .convert_to(desired_temperature_units);
        temperature_read
    }
}

#[derive(World, Debug)]
struct TestContext {
    temperature_reader: TestTemperatureReader,

    temperature_read: Temperature,
}

impl Default for TestContext {
    fn default() -> Self {
        Self {
            temperature_reader: TestTemperatureReader::new(Temperature::Celsius(0.0)),

            temperature_read: Temperature::Celsius(0.0),
        }
    }
}

fn parse_temperature_units(temperature_units_specified: String) -> TemperatureUnits {
    let parsed_temperature_units = match temperature_units_specified.as_str() {
        "Fahrenheit" => TemperatureUnits::Fahrenheit,
        "Celsius" => TemperatureUnits::Celsius,
        _ => panic!(
            "Unsupported temperature units: {}",
            temperature_units_specified
        ),
    };
    parsed_temperature_units
}

#[given(regex = r"an outside temperature of (.+) (.+),")]
fn given_outside_temperature(
    test_context: &mut TestContext,
    outside_temperature: f32,
    outside_temp_units: String,
) {
    let outside_temperature_units = parse_temperature_units(outside_temp_units);
    let outside_temperature = match outside_temperature_units {
        TemperatureUnits::Fahrenheit => Temperature::Fahrenheit(outside_temperature),
        TemperatureUnits::Celsius => Temperature::Celsius(outside_temperature),
    };

    test_context.temperature_reader = TestTemperatureReader::new(outside_temperature);
}

#[when(regex = r"the temperature is read in (.+),")]
fn when_temperature_read(test_context: &mut TestContext, temperature_units_specified: String) {
    let temperature_units = parse_temperature_units(temperature_units_specified);

    let temperature_reader = &mut test_context.temperature_reader;
    let temperature_read = temperature_reader.read_temperature(temperature_units);
    test_context.temperature_read = round_to_two_decimal_places(temperature_read);
}

#[then(regex = r"the temperature should be (.+) (.+).")]
fn verify_temperature_in_celsius(
    test_context: &mut TestContext,
    expected_temperature: f32,
    temperature_units_specified: String,
) {
    let temperature_units = parse_temperature_units(temperature_units_specified);

    let expected_temperature = match temperature_units {
        TemperatureUnits::Celsius => {
            round_to_two_decimal_places(Temperature::Celsius(expected_temperature))
        }
        TemperatureUnits::Fahrenheit => {
            round_to_two_decimal_places(Temperature::Fahrenheit(expected_temperature))
        }
    };
    let actual_temperature = test_context.temperature_read;
    assert_eq!(expected_temperature, actual_temperature);
}

fn main() {
    futures::executor::block_on(TestContext::run(
        "tests/features/temperature_reading.feature",
    ));
}
