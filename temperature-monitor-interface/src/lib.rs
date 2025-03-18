#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bit {
    Zero,
    One,
}

pub trait BitReader {
    fn read_next_bit(&mut self) -> Option<Bit>;
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

/// Returns a Byte shifted in either big or little endian.
fn shift_byte(byte_read: u8, order: &Endian) -> u8 {
    let mut byte_shifted = byte_read;
    byte_shifted = match order {
        Endian::Little => byte_shifted >> 1,
        Endian::Big => byte_shifted << 1,
    };

    byte_shifted
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
        let mut num_bits_read = 0;
        for _i in 0..num_bits {
            let bit_read = self.bit_reader.read_next_bit();
            if bit_read.is_none() {
                break;
            }

            let bit_read = bit_to_number(bit_read.unwrap());
            num_bits_read += 1;

            byte_read = add_bit_to(byte_read, bit_read, order);
            if num_bits_read == num_bits - 1 {
                break;
            }

            byte_read = shift_byte(byte_read, order);
        }

        if let Endian::Big = order {
            return byte_read;
        }

        // For Little-endian bit-reading, we prepend every bit at
        // the beginning (left side) of a byte to honor the ordering, but this
        // results in a larger number than read.
        //
        // Because of this, we have to scoot all of the bits read so far
        // down to the end (right side) of the byte.
        let shifts_left_to_do = (num_bits - 1) - num_bits_read;
        for _i in 0..shifts_left_to_do {
            byte_read = shift_byte(byte_read, order);
        }

        byte_read
    }
}
