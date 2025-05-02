extern crate find_folder;
extern crate piston_window;
extern crate rand;

mod draw;
mod game;
mod snake;

use piston_window::types::Color;
use piston_window::*;

use draw::to_coordinate_u32;
use game::Game;

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
    let (width, height) = (25, 25); //Window Size

    let mut game_window: PistonWindow = WindowSettings::new(
        "Snake Game",
        [to_coordinate_u32(width), to_coordinate_u32(height)],
    ) //cells to pixel
    .exit_on_esc(true) // press Esc to end the game
    .build()
    .unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();

    let font_path = assets.join("Karma_Future.otf");
    let mut glyphs = game_window.load_font(font_path).unwrap(); // Text extraction & rendering

    let mut game = Game::new(width, height);

    while let Some(event) = game_window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            // keyboard input
            game.key_pressed(key);
        }
        game_window.draw_2d(&event, |context, graphics, device| {
            // Game rendering loop
            clear(BACK_COLOR, graphics);
            game.draw(&context, graphics, &mut glyphs);

            glyphs.factory.encoder.flush(device); // properly rendered and updated when your score changes.
        });

        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}
