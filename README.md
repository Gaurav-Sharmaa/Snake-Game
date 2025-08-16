# 🐍 Snake Game in Rust

A classic Snake game implementation in Rust using the Piston game engine. Control the snake, eat the food, and grow as long as possible without hitting the walls or yourself!

![Snake Game Screenshot](assets/screenshot.png) *Screenshot coming soon*

## 🎮 Features

- Classic Snake gameplay with smooth controls
- Score tracking
- Game over detection (hitting walls or yourself)
- Retro-style font rendering
- Cross-platform support (Windows, macOS, Linux)

## 🚀 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version recommended)
- Cargo (comes with Rust installation)

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/Gaurav-Sharmaa/Snake-Game.git
   cd Snake-Game
   ```

2. Build the game:
   ```bash
   cargo build --release
   ```

### Running the Game

```bash
cargo run --release
```

## 🎯 How to Play

- Use **arrow keys** to control the snake's direction
- Eat the food (red square) to grow longer and increase your score
- Avoid hitting the walls or yourself
- Try to achieve the highest score possible!

## 🛠️ Project Structure

- `src/main.rs` - Entry point and game loop
- `src/game.rs` - Core game logic
- `src/snake.rs` - Snake implementation
- `src/draw.rs` - Drawing utilities
- `assets/` - Game assets including fonts

## 📦 Dependencies

- `piston_window` - Window and graphics handling
- `rand` - Random number generation for food placement
- `find_folder` - Asset loading

## 🤝 Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with ❤️ using Rust
- Inspired by the classic Snake game
- Uses [Piston](http://www.piston.rs/) game engine

---

Made with 🦀 by [Your Name] | [GitHub](https://github.com/Gaurav-Sharmaa)
