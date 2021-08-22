#[derive(Debug, Copy, Clone)]
pub struct Intensity {
    r: f32,
    g: f32,
    b: f32,
}

impl Intensity {
    pub fn new(r: f32, g: f32, b: f32) -> Intensity {
        assert!(r >= 0.);
        assert!(g >= 0.);
        assert!(b >= 0.);
        Intensity { r, g, b }
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

    pub fn zero() -> Intensity {
        Intensity::new(0., 0., 0.)
    }
}
