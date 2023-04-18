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
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}


use cxx::CxxString;
pub struct rbxfunctions {
    pub address: usize
}

impl rbxfunctions {
    pub unsafe fn GetName(&mut self) -> String {
        let name_location = *&*crate::cast!(
            *crate::cast!(self.address + 0x4, usize),
            usize
        ) as *const usize;
        let name = &*crate::cast!(name_location, CxxString);
        name.to_str().unwrap().to_string()
    }
    
}