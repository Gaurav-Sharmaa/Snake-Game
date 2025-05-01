use crate::draw::to_coordinate;
use piston_window::types::Color;
use piston_window::*;
use piston_window::{Glyphs, text};

use rand::{Rng, thread_rng};

use crate::draw::{draw_block, draw_rectangle};
use crate::snake::{Direction, Snake};

const FOOD_COLOR: Color = [0.80, 0.00, 0.00, 1.0];
const BORDER_COLOR: Color = [0.00, 0.00, 0.00, 1.0];
const GAMEOVER_COLOR: Color = [0.6, 0.5, 0.8, 0.7]; // soft purple, 70% opacity


const MOVING_PERIOD: f64 = 0.2; //FPS
const RESTART_TIME: f64 = 1.0; // 1sec restart time

pub struct Game {
    snake: Snake,

    food_exists: bool,
    food_x: i32,
    food_y: i32,

    width: i32,
    height: i32,

    game_over: bool,
    waiting_time: f64,

    score: u32,
    paused: bool,

    countdown_active: bool,
    countdown_value: i32,
    countdown_timer: f64,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            snake: Snake::new(2, 2),
            waiting_time: 0.0,
            food_exists: true,
            food_x: 6,
            food_y: 4,
            width,
            height,
            game_over: false,
            score: 0,
            paused: false,
            countdown_active: false,
            countdown_value: 3,
            countdown_timer: 0.0,
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        if key == Key::Space {
            if self.paused {
                // If currently paused, start countdown to resume
                self.countdown_active = true;
                self.countdown_value = 3;
                self.countdown_timer = 0.0;
            } else {
                // If currently playing, pause immediately
                self.paused = true;
            }
            return;
        }

        if self.game_over || self.paused || self.countdown_active {
            //do nothing if paused
            return;
        }

        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => None,
        };

        if dir.unwrap() == self.snake.head_direction().opposite() {
            return;
        }

        self.update_snake(dir);
    }

    pub fn draw(&self, context: &Context, graphics: &mut G2d, glyphs: &mut Glyphs) {
        self.snake.draw(context, graphics);

        if self.food_exists {
            draw_block(FOOD_COLOR, self.food_x, self.food_y, context, graphics);
        }

        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, context, graphics); // Horizontal Top Line 
        draw_rectangle(
            BORDER_COLOR,
            0,
            self.height - 1,
            self.width,
            1,
            context,
            graphics,
        ); // Horizontal Bottom Line
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, context, graphics); // Vertical Left Line
        draw_rectangle(
            BORDER_COLOR,
            self.width - 1,
            0,
            1,
            self.height,
            context,
            graphics,
        ); // Vertical Right Line

        let score_text = format!("{}", self.score); //Score
        text::Text::new_color([1.0, 1.0, 1.0, 1.0], 22) //White Text
            .draw(
                &score_text,
                glyphs,
                &context.draw_state,
                context.transform.trans(8.0, 22.0),
                graphics,
            )
            .unwrap_or_else(|_| ()); // used this to ignore errors

        // Draw countdown if active
        if self.countdown_active {
            let countdown_text = format!("{}", self.countdown_value);
            text::Text::new_color([1.0, 0.3, 0.0, 1.0], 72) // Large orange countdown
                .draw(
                    &countdown_text,
                    glyphs,
                    &context.draw_state,
                    // Center the countdown on screen
                    context.transform.trans(
                        to_coordinate(self.width / 2) - 20.0,
                        to_coordinate(self.height / 2) + 20.0,
                    ),
                    graphics,
                )
                .unwrap_or_else(|_| ());
        }
        // Draw paused message
        else if self.paused {
            let pause_text = "PAUSED - Press SPACE";
            text::Text::new_color([1.0, 1.0, 0.0, 1.0], 32)
                .draw(
                    pause_text,
                    glyphs,
                    &context.draw_state,
                    context.transform.trans(
                        to_coordinate(self.width / 2) - 150.0,
                        to_coordinate(self.height / 2),
                    ),
                    graphics,
                )
                .unwrap_or_else(|_| ());
        }

        if self.game_over {
            draw_rectangle(
                GAMEOVER_COLOR,
                0,
                0,
                self.width,
                self.height,
                context,
                graphics,
            );
        }

        if self.game_over {
            //  game over overlay 
            draw_rectangle(
                GAMEOVER_COLOR,
                0,
                0,
                self.width,
                self.height,
                context,
                graphics,
            );

            // Display the final score message
            let final_score_text = format!("Your Score is {}.", self.score);
            text::Text::new_color([1.0, 1.0, 1.0, 1.0], 40) // White, size 40
                .draw(
                    &final_score_text,
                    glyphs,
                    &context.draw_state,
                    context.transform.trans(
                        to_coordinate(self.width / 2) - 150.0, // Center horizontally 
                        to_coordinate(self.height / 2),
                    ),
                    graphics,
                )
                .unwrap_or_else(|_| ());
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        // Handle countdown if active
        if self.countdown_active {
            self.countdown_timer += delta_time;

            // Decrease countdown every second
            if self.countdown_timer >= 1.0 {
                self.countdown_value -= 1;
                self.countdown_timer = 0.0;

                // When countdown reaches zero, resume the game
                if self.countdown_value <= 0 {
                    self.countdown_active = false;
                    self.paused = false;
                }
            }
            return;
        }

        if self.paused {
            return; // Do nothing if paused
        }
        self.waiting_time += delta_time;

        if self.game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
        }

        if !self.food_exists {
            self.add_food();
        }

        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None);
        }
    }

    fn check_eating(&mut self) {
        let (head_x, head_y): (i32, i32) = self.snake.head_position();
        if self.food_exists && self.food_x == head_x && self.food_y == head_y {
            //food and head overlap then
            self.food_exists = false; // apple udha diya
            self.snake.restore_tail(); // snake grows
            self.score += 1; //increase the score
        }
    }

    fn check_if_snake_alive(&self, dir: Option<Direction>) -> bool {
        let (next_x, next_y) = self.snake.next_head(dir);

        if self.snake.overlap_tail(next_x, next_y) {
            return false; // kudh ko kat liya
        }

        next_x > 0 && next_y > 0 && next_x < self.width - 1 && next_y < self.height - 1 //wall collision
    }

    fn add_food(&mut self) {
        let mut rng = thread_rng();

        let mut new_x = rng.gen_range(1..self.width - 1); // exclude the boarder
        let mut new_y = rng.gen_range(1..self.height - 1);

        while self.snake.overlap_tail(new_x, new_y) {
            // No overlaping on snake
            new_x = rng.gen_range(1..self.width - 1);
            new_y = rng.gen_range(1..self.height - 1);
        }

        self.food_x = new_x;
        self.food_y = new_y;
        self.food_exists = true;
    }

    fn update_snake(&mut self, dir: Option<Direction>) {
        if self.check_if_snake_alive(dir) {
            self.snake.move_forward(dir);
            self.check_eating();
        } else {
            self.game_over = true;
        }
        self.waiting_time = 0.0;
    }

    fn restart(&mut self) {
        self.snake = Snake::new(2, 2);
        self.waiting_time = 0.0;
        self.food_exists = true;
        self.food_x = 6;
        self.food_y = 4;
        self.game_over = false;
        self.score = 0;
    }
}
