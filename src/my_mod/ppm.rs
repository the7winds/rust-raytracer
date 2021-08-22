use std::io;

pub trait SavableToPPM {
    fn save_to_ppm(&self, filename: &str) -> io::Result<()>;
}
