use cucumber::{World, given, then, when};

use temperature_monitor_interface::{Temperature, TemperatureReading, TemperatureUnits};

fn round_to_two_decimal_places(temperature: Temperature) -> Temperature {
    let reading = temperature.get_reading();
    let rounded_reading = (reading * 100.0).round() / 100.0;
    let rounded_temperature = Temperature::new(*temperature.get_units(), rounded_reading);
    rounded_temperature
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
            temperature_reader: TestTemperatureReader::new(Temperature::new(
                TemperatureUnits::Celsius,
                0.0,
            )),

            temperature_read: Temperature::new(TemperatureUnits::Celsius, 0.0),
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

fn parse_temperature(temperature_units_specified: String, desired_temperature: f32) -> Temperature {
    let temperature_units = parse_temperature_units(temperature_units_specified);

    Temperature::new(temperature_units, desired_temperature)
}

#[given(regex = r"an outside temperature of (.+) (.+),")]
fn given_outside_temperature(
    test_context: &mut TestContext,
    outside_temperature: f32,
    outside_temp_units: String,
) {
    let outside_temperature = parse_temperature(outside_temp_units, outside_temperature);
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
    let expected_temperature = parse_temperature(temperature_units_specified, expected_temperature);
    let actual_temperature = test_context.temperature_read;
    assert_eq!(expected_temperature, actual_temperature);
}

fn main() {
    futures::executor::block_on(TestContext::run(
        "tests/features/temperature_reading.feature",
    ));
}
