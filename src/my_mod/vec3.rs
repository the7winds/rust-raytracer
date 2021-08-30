// #[derive(Debug, Copy, Clone, PartialEq)]
// pub struct Vec3 {
//     pub x: f32,
//     pub y: f32,
//     pub z: f32,
// }

use glam::Vec3;

pub fn zero() -> Vec3 {
    Vec3::default()
}

pub fn refract(input_dir: Vec3, n: Vec3, angle_ratio: f32) -> Option<Vec3> {
    let input_cos = Vec3::dot(-input_dir, n);
    let input_forward = input_dir + n * input_cos;
    let output_forward = angle_ratio * input_forward;
    // impossible to increase angle more
    if output_forward.length() > 1. {
        None
    } else {
        let output_cos = (1. - output_forward.length_squared()).sqrt();
        let output_n = -n * output_cos;
        Some(output_forward + output_n)
    }
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2. * Vec3::dot(v, n) * n
}
