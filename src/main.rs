use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
};
use rand::seq::SliceRandom;
use std::io::{self, Write};

const CODE_LENGTH: usize = 4;
const MAX_ATTEMPTS: usize = 10;
const COLORS: [char; 6] = ['R', 'G', 'B', 'Y', 'M', 'C']; // Red, Green, Blue, Yellow, Magenta, Cyan

/// Represents the feedback for a guess
#[derive(Debug, PartialEq)]
struct Feedback {
    exact_matches: usize,  // Correct color in correct position
    color_matches: usize,  // Correct color in wrong position
}

/// Main game state
struct Game {
    secret_code: Vec<char>,
    attempts: usize,
}

impl Game {
    /// Create a new game with a random secret code
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let secret_code: Vec<char> = (0..CODE_LENGTH)
            .map(|_| *COLORS.choose(&mut rng).unwrap())
            .collect();

        Game {
            secret_code,
            attempts: 0,
        }
    }

    /// Validate a guess string
    fn validate_guess(&self, guess: &str) -> Result<Vec<char>, String> {
        if guess.len() != CODE_LENGTH {
            return Err(format!(
                "Invalid length! Please enter exactly {} colors.",
                CODE_LENGTH
            ));
        }

        let guess_upper: Vec<char> = guess.to_uppercase().chars().collect();

        for &ch in &guess_upper {
            if !COLORS.contains(&ch) {
                return Err(format!(
                    "Invalid color '{}'. Use only: {}",
                    ch,
                    COLORS.iter().collect::<String>()
                ));
            }
        }

        Ok(guess_upper)
    }

    /// Calculate feedback for a guess
    fn get_feedback(&self, guess: &[char]) -> Feedback {
        let mut exact_matches = 0;
        let mut secret_remaining = Vec::new();
        let mut guess_remaining = Vec::new();

        // First pass: find exact matches
        for i in 0..CODE_LENGTH {
            if guess[i] == self.secret_code[i] {
                exact_matches += 1;
            } else {
                secret_remaining.push(self.secret_code[i]);
                guess_remaining.push(guess[i]);
            }
        }

        // Second pass: find color matches (right color, wrong position)
        let mut color_matches = 0;
        for &color in &guess_remaining {
            if let Some(pos) = secret_remaining.iter().position(|&c| c == color) {
                color_matches += 1;
                secret_remaining.remove(pos);
            }
        }

        Feedback {
            exact_matches,
            color_matches,
        }
    }

    /// Play one round of the game
    fn play_round(&mut self, guess: Vec<char>) -> bool {
        self.attempts += 1;
        let feedback = self.get_feedback(&guess);

        // Display the guess with colors
        print!("  Guess {}: ", self.attempts);
        for &color in &guess {
            print_colored_symbol(color);
            print!(" ");
        }

        // Display feedback
        println!("\n  → {} exact, {} color{}",
            feedback.exact_matches,
            feedback.color_matches,
            if feedback.color_matches != 1 { "s" } else { "" }
        );

        // Check if won
        if feedback.exact_matches == CODE_LENGTH {
            return true;
        }

        // Give encouraging hints
        if self.attempts < MAX_ATTEMPTS {
            print_hint(feedback, self.attempts);
        }

        false
    }

    /// Reveal the secret code
    fn reveal_code(&self) {
        print!("  The code was: ");
        for &color in &self.secret_code {
            print_colored_symbol(color);
            print!(" ");
        }
        println!();
    }
}

/// Print a colored symbol based on the color character
fn print_colored_symbol(color_char: char) {
    let color = match color_char {
        'R' => Color::Red,
        'G' => Color::Green,
        'B' => Color::Blue,
        'Y' => Color::Yellow,
        'M' => Color::Magenta,
        'C' => Color::Cyan,
        _ => Color::White,
    };

    execute!(
        io::stdout(),
        SetForegroundColor(color),
        Print(format!("●")),
        ResetColor
    ).unwrap();
}

/// Print encouraging hints based on feedback
fn print_hint(feedback: Feedback, attempts: usize) {
    let Feedback { exact_matches, color_matches } = feedback;

    let message = match (exact_matches, color_matches) {
        (0, 0) => "  💭 Hmm, try completely different colors!",
        (0, _) => "  💡 You have the right colors, just wrong positions!",
        (1, _) => "  🎯 Getting warmer! One's in the right spot!",
        (2, _) => "  🔥 Nice! Two are perfectly placed!",
        (3, _) => "  ⚡ So close! Just one more to go!",
        _ => "  🎲 Keep analyzing the patterns...",
    };

    println!("{}", message);

    if attempts >= MAX_ATTEMPTS - 2 {
        println!("  ⏰ Running out of guesses!");
    }
}

/// Display the welcome banner
fn print_welcome() {
    println!("\n╔════════════════════════════════════════════╗");
    println!("║          🧩 CIPHERMIND 🧩                 ║");
    println!("║   The Ultimate Code-Breaking Challenge    ║");
    println!("╚════════════════════════════════════════════╝\n");

    println!("🎮 How to Play:");
    println!("  • I've created a secret 4-color code");
    println!("  • Available colors: ", );
    print!("    ");
    for &color in &COLORS {
        print_colored_symbol(color);
        print!(" = {} ", color);
    }
    println!("\n  • You have {} guesses to crack it!", MAX_ATTEMPTS);
    println!("  • After each guess, I'll tell you:");
    println!("    - How many are EXACT (right color, right position)");
    println!("    - How many are COLOR matches (right color, wrong position)");
    println!("\n💡 Example: Enter your guess as 4 letters, like: RGYB\n");
}

/// Ask if the player wants to play again
fn play_again() -> bool {
    print!("\n🔄 Play again? (y/n): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    matches!(input.trim().to_lowercase().as_str(), "y" | "yes")
}

/// Main game loop
fn main() {
    loop {
        print_welcome();

        let mut game = Game::new();
        let mut won = false;

        // Main guessing loop
        while game.attempts < MAX_ATTEMPTS && !won {
            print!("\n🎯 Enter your guess (or 'quit' to exit): ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();

            // Check for quit
            if input.eq_ignore_ascii_case("quit") {
                println!("\n👋 Thanks for playing!");
                game.reveal_code();
                return;
            }

            // Validate and process guess
            match game.validate_guess(input) {
                Ok(guess) => {
                    won = game.play_round(guess);
                }
                Err(error) => {
                    println!("  ❌ {}", error);
                    continue;
                }
            }
        }

        // Game over - show result
        println!("\n═══════════════════════════════════════════");
        if won {
            println!("🎉 CONGRATULATIONS! 🎉");
            println!("You cracked the code in {} {}!",
                game.attempts,
                if game.attempts == 1 { "guess" } else { "guesses" }
            );

            // Add special messages for exceptional performance
            match game.attempts {
                1 => println!("🏆 INCREDIBLE! A hole-in-one!"),
                2..=3 => println!("⭐ AMAZING! You're a master codebreaker!"),
                4..=6 => println!("✨ EXCELLENT! Great logical thinking!"),
                _ => println!("👍 Well done!"),
            }
        } else {
            println!("💥 GAME OVER!");
            println!("You've used all {} attempts.", MAX_ATTEMPTS);
            game.reveal_code();
            println!("\n🧠 Better luck next time! Each game is a new puzzle.");
        }
        println!("═══════════════════════════════════════════");

        // Ask to play again
        if !play_again() {
            println!("\n👋 Thanks for playing CipherMind!");
            println!("Remember: Logic conquers all codes! 🧩\n");
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feedback_all_exact() {
        let game = Game {
            secret_code: vec!['R', 'G', 'B', 'Y'],
            attempts: 0,
        };
        let feedback = game.get_feedback(&['R', 'G', 'B', 'Y']);
        assert_eq!(feedback.exact_matches, 4);
        assert_eq!(feedback.color_matches, 0);
    }

    #[test]
    fn test_feedback_no_matches() {
        let game = Game {
            secret_code: vec!['R', 'G', 'B', 'Y'],
            attempts: 0,
        };
        let feedback = game.get_feedback(&['M', 'M', 'C', 'C']);
        assert_eq!(feedback.exact_matches, 0);
        assert_eq!(feedback.color_matches, 0);
    }

    #[test]
    fn test_feedback_color_matches() {
        let game = Game {
            secret_code: vec!['R', 'G', 'B', 'Y'],
            attempts: 0,
        };
        let feedback = game.get_feedback(&['Y', 'B', 'G', 'R']);
        assert_eq!(feedback.exact_matches, 0);
        assert_eq!(feedback.color_matches, 4);
    }

    #[test]
    fn test_feedback_mixed() {
        let game = Game {
            secret_code: vec!['R', 'G', 'B', 'Y'],
            attempts: 0,
        };
        let feedback = game.get_feedback(&['R', 'B', 'Y', 'M']);
        assert_eq!(feedback.exact_matches, 1); // R in position 0
        assert_eq!(feedback.color_matches, 2); // B and Y in wrong positions
    }

    #[test]
    fn test_validate_guess_valid() {
        let game = Game::new();
        assert!(game.validate_guess("RGYB").is_ok());
        assert!(game.validate_guess("rgyb").is_ok()); // case insensitive
    }

    #[test]
    fn test_validate_guess_invalid_length() {
        let game = Game::new();
        assert!(game.validate_guess("RGB").is_err());
        assert!(game.validate_guess("RGBYY").is_err());
    }

    #[test]
    fn test_validate_guess_invalid_color() {
        let game = Game::new();
        assert!(game.validate_guess("RGBX").is_err());
    }
}
