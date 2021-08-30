use rand::random;

use crate::my_mod::{image::Image, rgb::RGB, vec3::Vec3};
use crate::my_mod::intensity::Intensity;
use crate::my_mod::resolution::Resolution;

impl From<Intensity> for Vec3 {
    fn from(intensity: Intensity) -> Self {
        Vec3::new(intensity.r(), intensity.g(), intensity.b())
    }
}

impl From<RGB> for Vec3 {
    fn from(rgb: RGB) -> Self {
        Vec3::new(rgb.r(), rgb.g(), rgb.b())
    }
}

pub fn random_from(min: f32, max: f32) -> f32 {
    random::<f32>() * (max - min) + min
}

pub fn random_on_unit_sphere() -> Vec3 {
    Vec3::new(
        random_from(-1., 1.),
        random_from(-1., 1.),
        random_from(-1., 1.),
    ).normalize()
}

pub fn random_in_unit_disk() -> (f32, f32) {
    (random_from(-1., 1.), random_from(-1., 1.))
}

#[allow(dead_code)]
pub fn test_image(width: usize, height: usize) -> Image {
    let mut image = Image::new(Resolution { width, height });

    for i in 0..height {
        for j in 0..width {
            let rgb = RGB::new(
                i as f32 / (image.width() - 1) as f32,
                j as f32 / (image.height() - 1) as f32,
                0.25,
            );

            image[(i, j)] = rgb;
        }
    }

    return image;
}
