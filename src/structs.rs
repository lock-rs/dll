#[derive(Debug)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn distance(&self, other: &Vector2) -> f32 {
        ((other.x - self.x).powi(2) + (other.y - self.y).powi(2)).sqrt()
    }
}

// Vector3
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn distance(&self, other: &Vector3) -> f32 {
        (
            (other.x - self.x).powi(2) +
            (other.y - self.y).powi(2) +
            (other.z - self.z).powi(2)
        ).sqrt()
    }
}

// Vector 4
pub struct Vector4 {
    pub X: f32,
    pub Y: f32,
    pub Z: f32,
    pub W: f32,
}