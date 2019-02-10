pub mod font57;

pub trait Font {
    fn get_char(c: char) -> Vec<u8>;
}