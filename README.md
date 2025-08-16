# 🐍 Snake Game in Rust

A classic Snake game implementation in Rust using the Piston game engine. Control the snake, eat the food, and grow as long as possible without hitting the walls or yourself!


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

---

<div align="center">
Made with 🦀 by | https://github.com/Gaurav-Sharmaa
</div>


