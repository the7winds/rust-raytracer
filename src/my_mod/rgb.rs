use crate::my_mod::intensity::Intensity;

#[derive(Debug, Copy, Clone)]
pub struct RGB {
    r: f32,
    g: f32,
    b: f32,
}

impl RGB {
    pub fn new(r: f32, g: f32, b: f32) -> RGB {
        assert!(0. <= r && r <= 1.);
        assert!(0. <= g && g <= 1.);
        assert!(0. <= b && b <= 1.);
        RGB { r, g, b }
    }

    pub fn r(&self) -> f32 {
        self.r
    }

    pub fn g(&self) -> f32 {
        self.g
    }

    pub fn b(&self) -> f32 {
        self.b
    }

    pub fn black() -> RGB {
        RGB {
            r: 0.,
            g: 0.,
            b: 0.,
        }
    }

    pub fn green() -> RGB {
        RGB {
            r: 0.,
            g: 1.,
            b: 0.,
        }
    }

    pub fn red() -> RGB {
        RGB {
            r: 1.,
            g: 0.,
            b: 0.,
        }
    }

    pub fn blue() -> RGB {
        RGB {
            r: 0.,
            g: 0.,
            b: 1.,
        }
    }

    pub fn white() -> RGB {
        RGB {
            r: 1.,
            g: 1.,
            b: 1.,
        }
    }
}

impl From<Intensity> for RGB {
    fn from(intence: Intensity) -> Self {
        RGB::new(
            intence.r().clamp(0., 1.),
            intence.g().clamp(0., 1.),
            intence.b().clamp(0., 1.),
        )
    }
}
