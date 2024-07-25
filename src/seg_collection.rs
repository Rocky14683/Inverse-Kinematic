use crate::segment::Segment;
use crate::vector::{*};
use raylib::core::drawing::{*};
pub struct SegCollection {
    segments: Vec<Segment>,
}

impl SegCollection {
    pub fn new() -> SegCollection {
        SegCollection {
            segments: Vec::new(),
        }
    }

    pub fn goto_position(&mut self, tx: f64, ty: f64, handle: &mut RaylibDrawHandle) {
        if self.segments.len() == 0 {
            return;
        }

        let mut pos = self.segments[0].follow(tx, ty);
        for seg in self.segments.iter_mut().skip(1){
            pos = seg.follow(pos.x, pos.y);
        }

        let last_idx = self.segments.len() - 1;

        self.segments[last_idx].update();
        self.segments[last_idx].show(handle);

        for seg in self.segments.iter_mut().rev() {
            seg.update();
            seg.show(handle);
        }
    }

    pub fn show_collection(&mut self, handle: &mut RaylibDrawHandle) {
        for seg in self.segments.iter() {
            seg.show(handle);
        }
    }

    pub fn change_start_point(&mut self, segment: &mut Segment, x: f64, y: f64) {
        segment.a.x = x;
        segment.a.y = y;
    }

    pub fn add_segment(&mut self, x: f64, y: f64, len: f64, angle: f64, thickness: f64) {
        self.segments.push(Segment::new(x, y, len, angle, thickness));
    }
}