#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn zero() -> Vec3 {
        Vec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }

    pub fn normalize(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn lerp(a: Vec3, b: Vec3, t: f32) -> Vec3 {
        assert!(0. <= t && t <= 1.);
        (1. - t) * a + t * b
    }

    pub fn dot(a: Vec3, b: Vec3) -> f32 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    pub fn cross(a: Vec3, b: Vec3) -> Vec3 {
        Vec3::new(
            a.y * b.z - a.z * b.y,
            a.z * b.x - a.x * b.z,
            a.x * b.y - a.y * b.x,
        )
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        let Vec3 { x, y, z } = self;
        x * x + y * y + z * z
    }

    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v - 2. * Vec3::dot(v, n) * n
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
}

impl core::ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl core::ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl core::ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-1f32 * rhs)
    }
}

impl core::ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl core::ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        self * (1. / rhs)
    }
}

impl core::ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::zero() - self
    }
}

impl core::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl core::ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}
