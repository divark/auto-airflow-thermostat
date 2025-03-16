mod bit_reader;

use cucumber::{World, given, then, when};

use crate::bit_reader::TestBitReader;
use temperature_monitor_interface::{ByteReader, Endian};

#[derive(Debug, Default, World)]
pub struct TestingEnvironment {
    bit_reader: TestBitReader,
    byte_reader: Option<ByteReader<TestBitReader>>,
    byte_read: u8,
}

#[given(regex = r"a Bit Reader reading the number ([0-9]+),")]
fn given_bit_reader_with_number(test_env: &mut TestingEnvironment, num_to_read: usize) {
    test_env.bit_reader = TestBitReader::new(num_to_read);
}

#[given("a Byte Reader that takes the Bit Reader,")]
fn given_byte_reader(test_env: &mut TestingEnvironment) {
    test_env.byte_reader = Some(ByteReader::new(test_env.bit_reader.clone()));
}

#[when("a byte is read in little-endian,")]
fn read_byte(test_env: &mut TestingEnvironment) {
    let reading_order = Endian::Little;
    test_env.byte_read = test_env.byte_reader.as_mut().unwrap().read(&reading_order);
}

#[then(regex = r"the result should be ([0-9]+).")]
fn verify_byte_read(test_env: &mut TestingEnvironment, expected_byte_read: u8) {
    let actual_byte_read = test_env.byte_read;
    assert_eq!(expected_byte_read, actual_byte_read);
}

fn main() {
    futures::executor::block_on(TestingEnvironment::run(
        "tests/features/byte_reader.feature",
    ));
}
