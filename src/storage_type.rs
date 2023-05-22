pub trait StorageType {
    fn representative_value(&self) -> f64;
    fn div_f64(&self, val: f64) -> Self;
}

impl StorageType for f32 {
    fn representative_value(&self) -> f64 {
        *self as f64
    }

    fn div_f64(&self, val: f64) -> Self {
        self / (val as f32)
    }
}

impl StorageType for f64 {
    fn representative_value(&self) -> f64 {
        *self
    }

    fn div_f64(&self, val: f64) -> Self {
        self / val
    }
}

#[cfg(feature = "glam-vec2")]
impl StorageType for glam::Vec2 {
    fn representative_value(&self) -> f64 {
        self.abs().max_element() as f64
    }

    fn div_f64(&self, val: f64) -> Self {
        *self / (val as f32)
    }
}

#[cfg(feature = "glam-vec3")]
impl StorageType for glam::Vec3 {
    fn representative_value(&self) -> f64 {
        self.abs().max_element() as f64
    }

    fn div_f64(&self, val: f64) -> Self {
        *self / (val as f32)
    }
}

#[cfg(feature = "glam-vec2")]
impl StorageType for glam::DVec2 {
    fn representative_value(&self) -> f64 {
        self.abs().max_element()
    }

    fn div_f64(&self, val: f64) -> Self {
        *self / val
    }
}

#[cfg(feature = "glam-vec3")]
impl StorageType for glam::DVec3 {
    fn representative_value(&self) -> f64 {
        self.abs().max_element()
    }

    fn div_f64(&self, val: f64) -> Self {
        *self / val
    }
}
