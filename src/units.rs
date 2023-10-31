pub mod length;
pub mod temperature;
pub mod mass;
pub mod volume;
pub trait Unit {
    fn by_name(name: &str) -> Self;
    fn to_string(&self) -> String;
    fn get_type(n: u32) -> Self;
}
