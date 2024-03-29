use glam::Vec3;
use rand::random;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::my_mod::bvh::BVHScene;
use crate::my_mod::camera::Camera;
use crate::my_mod::hittable::{Accuracy, Hittable};
use crate::my_mod::image::Image;
use crate::my_mod::intensity::Intensity;
use crate::my_mod::material::ScatteringResult;
use crate::my_mod::ray::Ray;
use crate::my_mod::resolution::Resolution;
use crate::my_mod::rgb::RGB;
use crate::my_mod::scene::Scene;
use crate::my_mod::time::{TimeInterval, TimePoint};
use crate::my_mod::utils::random_from;
use crate::my_mod::vec3;

pub struct Renderer {
    samples_per_pixel: usize,
    accuracy: Accuracy,
    max_depth: usize,
    camera: Camera,
    background: Intensity,
    resolution: Resolution,
}

impl Renderer {
    pub fn new(camera: Camera, resolution: Resolution) -> Self {
        Self {
            samples_per_pixel: 100,
            accuracy: Accuracy { min: 0.001, max: f32::INFINITY },
            max_depth: 20,
            camera,
            background: Intensity::new(1., 1., 1.),
            resolution,
        }
    }

    pub fn samples_per_pixel(mut self, samples_per_pixel: usize) -> Self {
        self.samples_per_pixel = samples_per_pixel;
        self
    }

    pub fn max_depth(mut self, max_depth: usize) -> Self {
        self.max_depth = max_depth;
        self
    }

    pub fn render(&self, scene: &Scene) -> Image {
        let Resolution { width, height } = self.resolution;
        let bvh = BVHScene::new(scene);

        let content = (0..height).into_par_iter().map(|row| {
            let mut row_pixels = vec![RGB::black(); width];

            for col in 0..width {
                let mut result_intensity = vec3::zero();

                for _ in 0..self.samples_per_pixel {
                    let u = ((col as f32) + random::<f32>()) / (width - 1) as f32;
                    let v = ((height - row - 1) as f32 + random::<f32>()) / (height - 1) as f32;
                    let ray = self.camera.get_ray(u, v);
                    let intense: Vec3 = ray_intensity(
                        &bvh,
                        &self.background,
                        &ray,
                        &self.accuracy,
                        self.max_depth
                    ).into();
                    result_intensity += intense;
                }
                result_intensity /= self.samples_per_pixel as f32;

                // gamma-correction
                let result_intensity = Vec3::new(
                    result_intensity.x.sqrt(),
                    result_intensity.y.sqrt(),
                    result_intensity.z.sqrt(),
                );

                let rgb = RGB::new(
                    result_intensity.x.clamp(0., 1.),
                    result_intensity.y.clamp(0., 1.),
                    result_intensity.z.clamp(0., 1.),
                );

                row_pixels[col] = rgb;
            }

            row_pixels
        }).flat_map(|x| x).collect();

        Image::new_with_content(
            Resolution {
                width,
                height
            },
            content)
    }
}

fn ray_intensity(
    hittable_list: &dyn Hittable,
    background: &Intensity,
    ray: &Ray,
    accuracy: &Accuracy,
    depth: usize,
) -> Intensity {
    if depth == 0 {
        return Intensity::zero();
    }

    match hittable_list.hit(ray, accuracy) {
        Some(hit_record) => match hit_record.material().scatter(&ray, &hit_record) {
            ScatteringResult::ScatterredRay(attenuation, scattered) => {
                let color =
                    ray_intensity(hittable_list, background, &scattered, accuracy, depth - 1);
                Intensity::new(
                    attenuation.r() * color.r(),
                    attenuation.g() * color.g(),
                    attenuation.b() * color.b(),
                )
            }
            ScatteringResult::Light(rgb) => rgb,
            ScatteringResult::None => Intensity::zero(),
        },
        None => *background,
    }
}

fn sample_time_point(frame: TimePoint, shutter: TimeInterval) -> TimePoint {
    let TimePoint(tp) = frame;
    let TimeInterval(interval) = shutter;
    TimePoint(tp + random_from(-interval / 2., interval / 2.))
}
