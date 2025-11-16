# 🧩 CipherMind

**A smart, terminal-based code-breaking puzzle game written in Rust**

CipherMind is a Mastermind-style logic puzzle that challenges players to crack a secret color code through deduction and strategic thinking. Perfect for quick brain-teasing sessions, this text-based game showcases Rust's performance, safety, and ability to create engaging experiences even without graphics.

## 🎮 Game Overview

The computer generates a secret 4-color code, and you have **10 attempts** to crack it. After each guess, you'll receive feedback that helps you narrow down the possibilities:

- **Exact matches**: Colors that are correct and in the right position
- **Color matches**: Colors that are correct but in the wrong position

Use logic and deduction to solve the puzzle before you run out of guesses!

## ✨ Features

- **🎨 Colorful Terminal UI**: Uses ANSI colors to display guesses and feedback visually
- **🧠 Smart Feedback System**: Precise hints after each guess to guide your deduction
- **💬 Encouraging Messages**: Fun, contextual hints that keep the game engaging
- **✅ Input Validation**: Robust error handling for invalid inputs
- **🔄 Replay System**: Generate a fresh puzzle for endless replayability
- **⚡ Fast & Safe**: Built with Rust for reliability and performance
- **🧪 Well-Tested**: Comprehensive unit tests for game logic

## 🚀 Quick Start

### Prerequisites

- Rust 1.70+ (install from [rustup.rs](https://rustup.rs))

### Installation & Running

```bash
# Clone the repository
git clone https://github.com/scottCodeGH/ciphermind-rust.git
cd ciphermind-rust

# Build and run
cargo run --release

# Or build first, then run
cargo build --release
./target/release/ciphermind
```

## 🎯 How to Play

1. **Start the game** - The computer generates a random 4-color code
2. **Available colors**:
   - **R** = Red
   - **G** = Green
   - **B** = Blue
   - **Y** = Yellow
   - **M** = Magenta
   - **C** = Cyan

3. **Make a guess** - Enter 4 letters (e.g., `RGYB`)
4. **Read the feedback**:
   - "2 exact" = 2 colors are correct and in the right position
   - "1 color" = 1 color is correct but in the wrong position

5. **Use logic** - Combine clues from all your guesses to deduce the code
6. **Win or learn** - Crack the code in 10 guesses or see the solution!

### Example Gameplay

```
🎯 Enter your guess: RGYB
  Guess 1: ● ● ● ●
  → 1 exact, 2 colors
  🎯 Getting warmer! One's in the right spot!

🎯 Enter your guess: RGBY
  Guess 2: ● ● ● ●
  → 2 exact, 1 color
  🔥 Nice! Two are perfectly placed!

🎯 Enter your guess: RGMY
  Guess 3: ● ● ● ●
  → 4 exact, 0 colors

🎉 CONGRATULATIONS! 🎉
You cracked the code in 3 guesses!
⭐ AMAZING! You're a master codebreaker!
```

## 🛠️ Technical Details

### Architecture

The game is structured with clean, modular Rust code:

- **`Game` struct**: Manages game state and logic
- **`Feedback` struct**: Represents the hint system
- **Validation**: Type-safe input handling with `Result<T, E>`
- **Randomization**: Uses `rand` crate for secure random code generation
- **Terminal UI**: `crossterm` for cross-platform colored output

### Key Functions

- `Game::new()` - Generates a random secret code
- `Game::validate_guess()` - Validates player input
- `Game::get_feedback()` - Calculates exact and color matches
- `Game::play_round()` - Processes one guess-feedback cycle

### Running Tests

The game includes comprehensive unit tests:

```bash
cargo test
```

Tests cover:
- Feedback calculation (all exact, no matches, color matches, mixed)
- Input validation (valid/invalid lengths and colors)
- Edge cases and game logic

## 🎓 Why Rust?

This project demonstrates Rust's strengths for game development:

- **Safety**: No null pointer errors or memory leaks
- **Performance**: Instant feedback and smooth terminal rendering
- **Error Handling**: Robust `Result` types for input validation
- **Cross-platform**: Runs on Linux, macOS, and Windows terminals
- **Modern tooling**: Cargo makes building and testing effortless

## 🎲 Game Strategy Tips

1. **Start systematic** - Try combinations that test different positions
2. **Track your clues** - Write down what each guess revealed
3. **Eliminate possibilities** - Use process of elimination
4. **Pattern recognition** - Look for patterns in the feedback
5. **Endgame precision** - When you're close, calculate the remaining possibilities

## 📝 License

This project is open source and available under the MIT License.

## 🤝 Contributing

Contributions are welcome! Feel free to:

- Report bugs
- Suggest new features
- Submit pull requests
- Improve documentation

## 🌟 Acknowledgments

Inspired by the classic Mastermind board game created by Mordecai Meirowitz in 1970. This digital adaptation brings the timeless logic puzzle to modern terminals with the power of Rust.

---

**Ready to test your deduction skills? Clone, build, and crack the code!** 🧩✨
