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
            let bit_read = match self.bit_reader.read_next_bit() {
                Some(Bit::Zero) => 0,
                None => 0,
                Some(Bit::One) => 1,
            };

            byte_read |= bit_read << 7;
            byte_read = byte_read >> 1;
        }

        let bit_read = match self.bit_reader.read_next_bit() {
            Some(Bit::Zero) => 0,
            None => 0,
            Some(Bit::One) => 1,
        };

        byte_read |= bit_read << 7;

        byte_read
    }
}
