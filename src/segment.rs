use crate::vector::Vector;
use raylib::core::drawing::RaylibDrawHandle;
use raylib::color::{*};
use raylib::drawing::RaylibDraw;

pub struct Segment {
    pub a: Vector,
    pub b: Vector,
    len: f64,
    angle: f64,
    thickness: f64,
}

impl Segment {
    pub fn new(x: f64, y: f64, len: f64, angle: f64, thickness: f64) -> Segment {
        Segment {
            a: Vector::new(x, y),
            b: Vector::new(0.0, 0.0),
            len,
            angle,
            thickness,
        }
    }

    pub fn follow(&mut self, tx: f64, ty: f64) -> Vector {
        let t = Vector::new(tx, ty);
        let dir = t.sub(&self.a);
        self.angle = dir.angle_of();

        self.a.x = t.x - self.len * self.angle.cos();
        self.a.y = t.y - self.len * self.angle.sin();

        Vector::new(self.a.x, self.a.y)
    }

    pub fn calculate_b(&mut self) {
        self.b.x = self.a.x + self.len * self.angle.cos();
        self.b.y = self.a.y + self.len * self.angle.sin();
    }

    pub fn update(&mut self) {
        self.calculate_b();
    }

    pub fn show(&self, handle: &mut RaylibDrawHandle) {
        handle.draw_line_ex(self.a.to_raylib_vector2(), self.b.to_raylib_vector2(), self.thickness as f32, Color::BLACK);
    }
}

