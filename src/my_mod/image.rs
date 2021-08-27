use std::io;

use crate::my_mod::rgb::RGB;

pub struct Image {
    width: usize,
    height: usize,
    content: Vec<RGB>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        Image {
            width,
            height,
            content: vec![RGB::black(); width * height],
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

impl core::ops::Index<(usize, usize)> for Image {
    type Output = RGB;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (row, column) = index;
        return &self.content[row * self.width + column];
    }
}

impl core::ops::IndexMut<(usize, usize)> for Image {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (row, column) = index;
        return &mut self.content[row * self.width + column];
    }
}

impl super::ppm::SavableToPPM for Image {
    fn save_to_ppm(&self, filename: &str) -> io::Result<()> {
        use std::io::Write;

        let mut f = std::fs::File::create(filename)?;

        f.write(format!("P3\n").as_bytes())?;
        f.write(format!("{} {}\n", self.width(), self.height()).as_bytes())?;
        f.write(format!("255\n").as_bytes())?;

        for i in 0..self.height() {
            for j in 0..self.width() {
                let rgb = self[(i, j)];
                let r = (255. * rgb.r()) as i32;
                let g = (255. * rgb.g()) as i32;
                let b = (255. * rgb.b()) as i32;

                f.write(format!("{} {} {}\n", r, g, b).as_bytes())?;
            }
        }

        Ok(())
    }
}