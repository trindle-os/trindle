use std::ops::Mul;

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Color32F([f32; 4]);

impl Color32F {
    #[inline]
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self([r, g, b, a])
    }
}

impl Color32F {
    pub const TRANSPARENT: Color32F = Color32F::new(0.0, 0.0, 0.0, 0.0);

    pub const BLACK: Color32F = Color32F::new(0f32, 0f32, 0f32, 1f32);
}

impl Color32F {
    #[inline]
    pub fn r(&self) -> f32 {
        self.0[0]
    }

    #[inline]
    pub fn g(&self) -> f32 {
        self.0[1]
    }

    #[inline]
    pub fn b(&self) -> f32 {
        self.0[2]
    }

    #[inline]
    pub fn a(&self) -> f32 {
        self.0[3]
    }

    #[inline]
    pub fn components(self) -> [f32; 4] {
        self.0
    }
}

impl Color32F {
    #[inline]
    pub fn is_opaque(&self) -> bool {
        self.a() == 1f32
    }
}

impl From<[f32; 4]> for Color32F {
    #[inline]
    fn from(value: [f32; 4]) -> Self {
        Self(value)
    }
}

impl Mul<f32> for Color32F {
    type Output = Color32F;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.r() * rhs, self.g() * rhs, self.b() * rhs, self.a() * rhs)
    }
}
