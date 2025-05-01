pub mod draw;
pub mod snake;

#[cfg(test)]
mod tests {
    use crate::snake::Snake;

    #[test]
    fn test_head_position() {
        let snake = Snake::new(4, 2);
        assert_eq!(snake.head_position(), (4, 2));
    }
}
