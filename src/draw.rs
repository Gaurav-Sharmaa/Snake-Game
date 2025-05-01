use piston_window::types::Color;
use piston_window::{Context, G2d, rectangle};

const BLOCK_SIZE: f64 = 25.0;

pub fn to_coordinate(game_coordinate: i32) -> f64 {
    (game_coordinate as f64) * BLOCK_SIZE // casted as f64 for getting exact coordinate in pixel
}

pub fn to_coordinate_u32(game_coordinate: i32) -> u32{
    to_coordinate(game_coordinate) as u32// get precise sol
}

pub fn draw_block(color: Color, x: i32, y: i32, context: &Context, graphics: &mut G2d) {
    let gui_x = to_coordinate(x);
    let gui_y = to_coordinate(y);

    rectangle(
        color,
        [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE], //width, height
        context.transform, // tells how and where to draw things — it includes position, rotation, and scale.
        graphics,
    );
}

pub fn draw_rectangle(
    color: Color,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    context: &Context,
    graphics: &mut G2d,
) {
    let x = to_coordinate(x);
    let y = to_coordinate(y);

    rectangle(
        color,
        [
            x,
            y,
            BLOCK_SIZE * (width as f64),
            BLOCK_SIZE * (height as f64),
        ],
        context.transform,
        graphics,
    );
}
