use cucumber::{World, given, then, when};

use temperature_monitor_interface::{Bit, BitReader};

/// What's more important is that the interface
/// behaves as-intended, since this sequence
/// of bits could be coming in from any source.
#[derive(Debug, Default, Clone)]
pub struct TestBitReader {
    number: usize,
}

impl TestBitReader {
    pub fn new(number: usize) -> Self {
        Self { number }
    }
}

impl BitReader for TestBitReader {
    fn read_next_bit(&mut self) -> Bit {
        if self.number == 0 {
            return Bit::Zero;
        }

        let mask = 1;
        let read_bit = self.number & mask;
        self.number = self.number >> 1;

        match read_bit {
            0 => Bit::Zero,
            1 => Bit::One,
            _ => panic!("read_next_bit: Masking failed. A number was found greater than 1."),
        }
    }
}

#[derive(Debug, Default, World)]
struct TestingEnvironment {
    number: usize,
    bits_read: Vec<Bit>,
}

#[given(regex = r"the number ([0-9]+),")]
fn given_some_number(test_env: &mut TestingEnvironment, given_number: usize) {
    test_env.number = given_number;
}

#[when(regex = r"the number is read for ([0-9]+) bits,")]
fn number_read_for_bits(test_env: &mut TestingEnvironment, num_bits_to_read: usize) {
    let mut bit_reader = TestBitReader::new(test_env.number);

    for _i in 0..num_bits_to_read {
        test_env.bits_read.push(bit_reader.read_next_bit());
    }
}

#[then(regex = r"there should be ([0-9]+) 1s found.")]
fn verify_num_of_1_bits_found(test_env: &mut TestingEnvironment, expected_num_1_bits: usize) {
    let actual_num_1_bits = test_env
        .bits_read
        .iter()
        .filter(|bit| *bit == &Bit::One)
        .count();

    assert_eq!(expected_num_1_bits, actual_num_1_bits);
}

#[then(regex = r"there should be ([0-9]+) 0s found.")]
fn verify_num_of_0_bits_found(test_env: &mut TestingEnvironment, expected_num_0_bits: usize) {
    let actual_num_0_bits = test_env
        .bits_read
        .iter()
        .filter(|bit| *bit == &Bit::Zero)
        .count();

    assert_eq!(expected_num_0_bits, actual_num_0_bits);
}

fn main() {
    futures::executor::block_on(TestingEnvironment::run("tests/features/bit_reader.feature"));
}
