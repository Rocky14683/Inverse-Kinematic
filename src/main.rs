mod segment;
mod vector;
mod seg_collection;

use raylib::drawing::*;


fn main() {
    const HEIGHT: i32 = 750;
    const WIDTH: i32 = 1250;

    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Segment Following")
        .build();
    // let mut seg = segment::Segment::new(0.0, 0.0, 200.0, 0.0, 5.0);
    let mut seg_collection = seg_collection::SegCollection::new();
    for i in 0..50 {
        seg_collection.add_segment(0.0, 0.0, 10.0, 0.0, 5.0);
    }

    while !rl.window_should_close() {
        let mouse_x = rl.get_mouse_x();
        let mouse_y = rl.get_mouse_y();

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(raylib::color::Color::WHITE);

        seg_collection.goto_position(mouse_x as f64, mouse_y as f64, &mut d);
    }
}
