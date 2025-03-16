#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bit {
    Zero,
    One,
}

pub trait BitReader {
    fn read_next_bit(&mut self) -> Bit;
}

pub enum Endian {
    Little,
    Big,
}

#[derive(Debug)]
pub struct ByteReader<U>
where
    U: BitReader,
{
    bit_reader: U,
}

/// Returns a byte where a bit has been added onto it,
/// depending on the Endian-ness.
fn add_bit_to(byte: u8, bit_read: u8, order: &Endian) -> u8 {
    let mut new_byte = byte;
    match order {
        Endian::Little => new_byte |= bit_read << 7,
        Endian::Big => new_byte |= bit_read,
    }

    new_byte
}

/// Returns a Bit read as a unsigned integer.
fn bit_to_number(read_bit: Bit) -> u8 {
    match read_bit {
        Bit::Zero => 0,
        Bit::One => 1,
    }
}

impl<U> ByteReader<U>
where
    U: BitReader,
{
    pub fn new(bit_reader: U) -> Self {
        Self { bit_reader }
    }

    pub fn read(&mut self, order: &Endian) -> u8 {
        let mut byte_read = 0;

        let num_bits = 8;
        for _i in 0..num_bits - 1 {
            let bit_read = bit_to_number(self.bit_reader.read_next_bit());

            byte_read = add_bit_to(byte_read, bit_read, order);
            byte_read = byte_read >> 1;
        }

        let bit_read = bit_to_number(self.bit_reader.read_next_bit());
        byte_read = add_bit_to(byte_read, bit_read, order);

        byte_read
    }
}
