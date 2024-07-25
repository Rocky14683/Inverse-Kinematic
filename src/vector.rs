use std::f32::consts::PI;
use raylib::ffi::Vector2;

pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64) -> Vector {
        Vector { x, y }
    }

    pub fn add(&self, other: &Vector) -> Vector {
        Vector::new(self.x + other.x, self.y + other.y)
    }

    pub fn sub(&self, other: &Vector) -> Vector {
        Vector::new(self.x - other.x, self.y - other.y)
    }

    pub fn get_mag(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn angle_of(&self) -> f64 {
        if self.x == 0_f64 {
            return 0_f64;
        }

        let mut angle = (self.y / self.x).atan();
        if self.y < 0.0 && self.x < 0.0 {
            angle += PI as f64;
        } else if self.x < 0.0 {
            angle += PI as f64;
        }
        angle
    }

    pub fn to_raylib_vector2(&self) -> Vector2 {
        Vector2 { x: self.x as f32, y: self.y as f32 }
    }
}