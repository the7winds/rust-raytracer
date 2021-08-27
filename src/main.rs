use std::error::Error;
use std::rc::Rc;

use rand::random;

use crate::my_mod::angle::Angle;
use crate::my_mod::animated_hittable::{AnimatedHittable, AnimatedHittableList};
use crate::my_mod::animated_sphere::AnimatedSphere;
use crate::my_mod::camera::Camera;
use crate::my_mod::hittable::Accuracy;
use crate::my_mod::image::Image;
use crate::my_mod::intensity::Intensity;
use crate::my_mod::material::Attenuation;
use crate::my_mod::material::dielectric::Dielectric;
use crate::my_mod::material::lambertian::Lambertian;
use crate::my_mod::material::metal::Metal;
use crate::my_mod::material::ScatteringResult;
use crate::my_mod::ppm::SavableToPPM;
use crate::my_mod::ray::Ray;
use crate::my_mod::rgb::RGB;
use crate::my_mod::time::{TimeInterval, TimePoint};
use crate::my_mod::utils::random_from;
use crate::my_mod::vec3::Vec3;

mod my_mod;

fn background(_: &Ray) -> Intensity {
    Intensity::new(1., 1., 1.)
}

fn ray_intensity(
    hittable_list: &AnimatedHittableList,
    background: impl Fn(&Ray) -> Intensity,
    time: TimePoint,
    ray: &Ray,
    accuracy: &Accuracy,
    depth: usize,
) -> Intensity {
    if depth == 0 {
        return Intensity::zero();
    }

    match hittable_list.hit(time, ray, accuracy) {
        Some(hit_record) => match hit_record.material().scatter(&ray, &hit_record) {
            ScatteringResult::ScatterredRay(attenuation, scattered) => {
                let color =
                    ray_intensity(hittable_list, background, time, &scattered, accuracy, depth - 1);
                Intensity::new(
                    attenuation.r() * color.r(),
                    attenuation.g() * color.g(),
                    attenuation.b() * color.b(),
                )
            }
            ScatteringResult::Light(rgb) => rgb,
            ScatteringResult::None => Intensity::zero(),
        },
        None => background(ray),
    }
}

fn get_scene() -> AnimatedHittableList {
    let mut list: Vec<Box<dyn AnimatedHittable>> = vec![];

    let sphere_ground = Box::new(
        AnimatedSphere::new(
            Vec3::new(0., -1000., 0.),
            1000.,
            Rc::new(Lambertian::new(Attenuation::new(0.5, 0.5, 0.5))),
            |_| Vec3::zero()
        ));

    list.push(sphere_ground);

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f32 = random();
            let center = Vec3::new(
                a as f32 + 0.9 * random::<f32>(),
                0.2,
                b as f32 + 0.9 * random::<f32>(),
            );

            if (center - Vec3::new(4., 0.2, 0.)).length() > 0.9 {
                let sphere = if choose_mat < 0.8 {
                    AnimatedSphere::new(
                        center,
                        0.2,
                        Rc::new(Lambertian::new(Attenuation::random())),
                        |TimePoint(time)| Vec3::new(0., time.sin(), 0.)
                    )
                } else if choose_mat < 0.95 {
                    AnimatedSphere::new(
                        center,
                        0.2,
                        Rc::new(Metal::new(Attenuation::random(), random())),
                        |_| Vec3::zero()
                    )
                } else {
                    AnimatedSphere::new(
                        center,
                        0.2,
                        Rc::new(Dielectric::new(1.5)),
                        |_| Vec3::zero()
                    )
                };
                list.push(Box::new(sphere));
            }
        }
    }

    list.push(Box::new(
        AnimatedSphere::new(
            Vec3::new(0., 1., 0.),
            1.0,
            Rc::new(Dielectric::new(1.5)),
            |_| Vec3::zero()
        )
    ));

    list.push(Box::new(
        AnimatedSphere::new(
            Vec3::new(-4., 1., 0.),
            1.0,
            Rc::new(Lambertian::new(Attenuation::new(0.5, 0.2, 0.1))),
            |_| Vec3::zero()
        )
    ));

    list.push(Box::new(
        AnimatedSphere::new(
            Vec3::new(4., 1.0, 0.),
            1.0,
            Rc::new(Metal::new(Attenuation::new(0.7, 0.6, 0.5), 0.)),
            |_| Vec3::zero())
    ));

    AnimatedHittableList { list }
}

fn sample_time_point(frame: TimePoint, shutter: TimeInterval) -> TimePoint {
    let TimePoint(tp) = frame;
    let TimeInterval(interval) = shutter;
    TimePoint(tp + random_from(-interval / 2., interval / 2.))
}

fn main() -> Result<(), Box<dyn Error>> {
    let samples_per_pixel = 100;
    let max_depth = 50;
    let ref accuracy = Accuracy {
        min: 0.001,
        max: f32::INFINITY,
    };
    let vfov = Angle::Degrees(60.0);
    let aspect_ratio = 3. / 2. as f32;
    let image_width = 500;
    let image_height = (image_width as f32 / aspect_ratio) as usize;

    let mut image = Image::new(image_width, image_height);

    // Camera
    let from = Vec3::new(13., 2., 3.);
    let at = Vec3::new(0., 0., 0.);
    let up = Vec3::new(0., 1., 0.);
    let focus_dist = 10.;
    let aperture = 0.1;
    let shutter = TimeInterval(0.5);
    let camera = Camera::new(
        &from,
        &at,
        &up,
        vfov,
        aspect_ratio,
        focus_dist,
        aperture,
        shutter
    );

    let mut hittable_list = get_scene();

    for row in 0..image_height {
        for col in 0..image_width {
            let mut result_intensity = Vec3::zero();
            for _ in 0..samples_per_pixel {
                let u = ((col as f32) + random::<f32>()) / (image_width - 1) as f32;
                let v =
                    ((image_height - row - 1) as f32 + random::<f32>()) / (image_height - 1) as f32;
                let ray = camera.get_ray(u, v);
                let time = sample_time_point(TimePoint(0.), camera.shutter());
                result_intensity +=
                    ray_intensity(&mut hittable_list, background, time, &ray, accuracy, max_depth).into();
            }
            result_intensity /= samples_per_pixel as f32;

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

            image[(row, col)] = rgb;
        }
    }

    image.save_to_ppm("image.ppm")?;

    Ok(())
}
