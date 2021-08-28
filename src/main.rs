use std::error::Error;
use std::rc::Rc;

use rand::random;

use crate::my_mod::angle::Angle;
use crate::my_mod::animated_hittable::{AnimatedHittable, AnimatedHittableList};
use crate::my_mod::animated_sphere::AnimatedSphere;
use crate::my_mod::camera::Camera;
use crate::my_mod::material::Attenuation;
use crate::my_mod::material::dielectric::Dielectric;
use crate::my_mod::material::lambertian::Lambertian;
use crate::my_mod::material::metal::Metal;
use crate::my_mod::ppm::SavableToPPM;
use crate::my_mod::renderer::Renderer;
use crate::my_mod::resolution::Resolution;
use crate::my_mod::time::{TimeInterval, TimePoint};
use crate::my_mod::vec3::Vec3;

mod my_mod;

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

fn main() -> Result<(), Box<dyn Error>> {
    let samples_per_pixel = 100;
    let max_depth = 50;

    let aspect_ratio = 3. / 2. as f32;
    let resolution = {
        let width = 500;
        let height = (width as f32 / aspect_ratio) as usize;
        Resolution { width, height }
    };

    // Camera
    let camera = {
        let vfov = Angle::Degrees(60.0);
        let from = Vec3::new(13., 2., 3.);
        let at = Vec3::new(0., 0., 0.);
        let up = Vec3::new(0., 1., 0.);
        let focus_dist = 10.;
        let aperture = 0.1;
        let shutter = TimeInterval(0.5);

        Camera::new(
            &from,
            &at,
            &up,
            vfov,
            aspect_ratio,
            focus_dist,
            aperture,
            shutter
        )
    };

    let world = get_scene();

    Renderer::new(camera, resolution)
        .samples_per_pixel(samples_per_pixel)
        .max_depth(max_depth)
        .show_progress(true)
        .render(&world)
        .save_to_ppm("image.ppm")?;

    Ok(())
}
