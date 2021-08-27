use crate::my_mod::angle::Angle;
use crate::my_mod::ray::Ray;
use crate::my_mod::time::TimeInterval;
use crate::my_mod::utils::random_in_unit_disk;
use crate::my_mod::vec3::Vec3;

#[derive(Debug)]
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lens_radius: f32,
    x: Vec3,
    y: Vec3,
    shutter: TimeInterval,
}

impl Camera {
    pub fn new(
        from: &Vec3,
        at: &Vec3,
        up: &Vec3,
        vfov: Angle,
        aspect_ratio: f32,
        focus_dist: f32,
        aperture: f32,
        shutter: TimeInterval
    ) -> Camera {
        let theta = vfov.radians();
        let h = (theta / 2.).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let z = (*from - *at).normalize();
        let x = Vec3::cross(*up, z).normalize();
        let y = Vec3::cross(z, x);

        let origin = *from;

        let horizontal = focus_dist * viewport_width * x;
        let vertical = focus_dist * viewport_height * y;
        let lower_left_corner = origin - horizontal / 2. - vertical / 2. - focus_dist * z;

        let lens_radius = aperture / 2.;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            lens_radius,
            x,
            y,
            shutter
        }
    }

    pub fn shutter(&self) -> TimeInterval {
        self.shutter
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let (rd_x, rd_y) = random_in_unit_disk();
        let rd_x = self.lens_radius * rd_x;
        let rd_y = self.lens_radius * rd_y;
        let offset = self.x * rd_x + self.y * rd_y;
        let origin = self.origin + offset;
        let direction =
            self.lower_left_corner + u * self.horizontal + v * self.vertical - origin;
        Ray::new(&origin, &direction)
    }
}
