#[derive(Debug, PartialEq, Eq)]
pub enum Bit {
    Zero,
    One,
}

pub trait BitReader {
    fn read_next_bit(&mut self) -> Bit;
}
