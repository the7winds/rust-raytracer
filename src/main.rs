use std::error::Error;
use std::rc::Rc;

use rand::random;

use crate::my_mod::angle::Angle;
use crate::my_mod::bvh::BoundableAndHittable;
use crate::my_mod::camera::Camera;
use crate::my_mod::hittable::HittableList;
use crate::my_mod::material::{Attenuation, Material};
use crate::my_mod::ppm::SavableToPPM;
use crate::my_mod::renderer::Renderer;
use crate::my_mod::resolution::Resolution;
use crate::my_mod::sphere::Sphere;
use crate::my_mod::vec3::Vec3;
use crate::my_mod::image::Image;

mod my_mod;

fn get_scene() -> HittableList {
    let mut list: Vec<Rc<dyn BoundableAndHittable>> = vec![];

    let sphere_ground = Rc::new(
        Sphere::new(
            Vec3::new(0., -1000., 0.),
            1000.,
            Material::lambertian(Attenuation::new(0.5, 0.5, 0.5)),
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
                    Sphere::new(
                        center,
                        0.2,
                        Material::lambertian(Attenuation::random()),
                    )
                } else if choose_mat < 0.95 {
                    Sphere::new(
                        center,
                        0.2,
                        Material::metal(Attenuation::random(), random()),
                    )
                } else {
                    Sphere::new(
                        center,
                        0.2,
                        Material::dielectric(1.5),
                    )
                };
                list.push(Rc::new(sphere));
            }
        }
    }

    list.push(Rc::new(
        Sphere::new(
            Vec3::new(0., 1., 0.),
            1.0,
            Material::dielectric(1.5),
        )
    ));

    list.push(Rc::new(
        Sphere::new(
            Vec3::new(-4., 1., 0.),
            1.0,
            Material::lambertian(Attenuation::new(0.5, 0.2, 0.1)),
        )
    ));

    list.push(Rc::new(
        Sphere::new(
            Vec3::new(4., 1.0, 0.),
            1.0,
            Material::metal(Attenuation::new(0.7, 0.6, 0.5), 0.)
        )
    ));

    HittableList { list }
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

        Camera::new(
            &from,
            &at,
            &up,
            vfov,
            aspect_ratio,
            focus_dist,
            aperture,
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
