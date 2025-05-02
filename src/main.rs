extern crate find_folder;
extern crate piston_window;
extern crate rand;

mod draw;
mod game;
mod snake;

use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink, Source};

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

    let font_path = assets.join("Retro_text.otf");
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

    let (_stream, stream_handle) = OutputStream::try_default().unwrap(); //Audio Output

    let eat_sound = BufReader::new(File::open("assets/eat.wav").unwrap());
    let gameover_sound = BufReader::new(File::open("assets/gameover.wav").unwrap());
    let bgm_file = BufReader::new(File::open("assets/in_game_background.mp3").unwrap());

    // Created a sink for bgm and play it in a loop
    let bgm_sink = Sink::try_new(&stream_handle).unwrap();
    let bgm_source = Decoder::new(bgm_file).unwrap().repeat_infinite();
    bgm_sink.append(bgm_source);
    bgm_sink.set_volume(0.2); // Softer Bgm 

    let mut game = Game::new(width, height, stream_handle.clone()); 

}
