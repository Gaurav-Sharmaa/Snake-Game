use piston_window::types::Color;
use piston_window::{Context, G2d};
use std::collections::LinkedList; // Quickly add or remove elements from both ends. Snake Tail

use crate::draw::draw_block;

pub const SNAKE_HEAD_COLOR: Color = [0.00, 0.75, 0.00, 1.00];
const SNAKE_BODY_COLOR: Color = [0.50, 0.90, 0.50, 1.00]; // Light green

#[derive(Copy, Clone, PartialEq)] // PartialEq is used for == and != on types.
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Debug, Clone)]
struct Block {
    x: i32,
    y: i32,
}

pub struct Snake {
    direction: Direction,
    body: LinkedList<Block>,
    tail: Option<Block>,
}

impl Snake {
    pub fn new(x: i32, y: i32) -> Snake {
        let mut body: LinkedList<Block> = LinkedList::new();
        body.push_back(Block { x: x + 2, y });
        body.push_back(Block { x: x + 1, y });
        body.push_back(Block { x: x, y });
        //(Tail) [x+2][x+1][x] (Head) → 🐍

        Snake {
            direction: Direction::Right,
            body,
            tail: None,
        }
    }

    pub fn draw(&self, context: &Context, graphics: &mut G2d) {
        let mut first = true;

        for block in &self.body {
            if first{
                // Draw the Side of the snake head
                crate::draw::draw_snake_head(block.x, block.y, context, graphics);
                first = false;
            } else {
                draw_block(SNAKE_BODY_COLOR, block.x, block.y, context, graphics);
            }
        }
    }

    pub fn head_position(&self) -> (i32, i32) {
        let head_block = self.body.front().unwrap();
        (head_block.x, head_block.y)
    }

    pub fn move_forward(&mut self, dir: Option<Direction>) {
        match dir {
            Some(d) => self.direction = d, // new dir is given move to that
            None => (),                    //else remain in the same dir
        }

        let (last_x, last_y) = self.head_position(); //getting the head posi in x & y 

        let new_block = match self.direction {
            Direction::Up => Block {
                x: last_x,
                y: last_y - 1,
            },
            Direction::Down => Block {
                x: last_x,
                y: last_y + 1,
            },
            Direction::Left => Block {
                x: last_x - 1,
                y: last_y,
            },
            Direction::Right => Block {
                x: last_x + 1,
                y: last_y,
            },
        };
        self.body.push_front(new_block); // added a new block in front of the body
        let removed_block = self.body.pop_back().unwrap(); // Removed the last block
        self.tail = Some(removed_block);
    }

    pub fn head_direction(&self) -> Direction {
        // It tells you which way the snake’s head is pointing.
        self.direction
    }

    pub fn next_head(&self, dir: Option<Direction>) -> (i32, i32) {
        let (head_x, head_y): (i32, i32) = self.head_position();

        let mut moving_direction = self.direction;
        match dir {
            Some(d) => moving_direction = d,
            None => {}
        }

        match moving_direction {
            Direction::Up => (head_x, head_y - 1),
            Direction::Down => (head_x, head_y + 1),
            Direction::Left => (head_x - 1, head_y),
            Direction::Right => (head_x + 1, head_y),
        }
    }

    pub fn restore_tail(&mut self) {
        // eat apple grow tail
        let blk = self.tail.clone().unwrap();
        self.body.push_back(blk);
    }

    pub fn overlap_tail(&self, x: i32, y: i32) -> bool {
        let mut checked = 0;
        for block in &self.body {
            if x == block.x && y == block.y {
                return true;
            }

            checked += 1;
            if checked == self.body.len() - 1 {
                break;
            }
        }
        return false;
    }
}
