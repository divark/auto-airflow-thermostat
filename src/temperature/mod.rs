pub mod station;

pub trait Client {}

pub trait Requests {
    fn read_client(&self) -> impl Client;
    fn is_empty(&self) -> bool;
}

pub enum TemperatureUnit {
    Fahrenheit,
    Celcius,
}
