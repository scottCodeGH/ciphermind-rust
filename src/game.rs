//! Core game logic for CipherMind — no terminal I/O lives here.

use rand::seq::SliceRandom;

/// Full color palette. Individual difficulties use a prefix of this list.
pub const COLORS: [char; 6] = ['R', 'G', 'B', 'Y', 'M', 'C']; // Red, Green, Blue, Yellow, Magenta, Cyan

/// A difficulty preset: how long the code is, how many colors are in play,
/// and how many guesses the player gets.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Difficulty {
    pub name: &'static str,
    pub code_length: usize,
    pub num_colors: usize,
    pub max_attempts: usize,
}

impl Difficulty {
    pub const EASY: Difficulty = Difficulty {
        name: "Easy",
        code_length: 4,
        num_colors: 4,
        max_attempts: 12,
    };
    pub const CLASSIC: Difficulty = Difficulty {
        name: "Classic",
        code_length: 4,
        num_colors: 6,
        max_attempts: 10,
    };
    pub const HARD: Difficulty = Difficulty {
        name: "Hard",
        code_length: 5,
        num_colors: 6,
        max_attempts: 8,
    };

    /// The three presets, in menu order.
    pub const ALL: [Difficulty; 3] = [Self::EASY, Self::CLASSIC, Self::HARD];

    /// The colors available at this difficulty (a prefix of [`COLORS`]).
    pub fn colors(&self) -> &'static [char] {
        &COLORS[..self.num_colors]
    }
}

/// Represents the feedback for a guess
#[derive(Debug, PartialEq)]
pub struct Feedback {
    pub exact_matches: usize, // Correct color in correct position
    pub color_matches: usize, // Correct color in wrong position
}

/// Main game state
pub struct Game {
    pub secret_code: Vec<char>,
    pub attempts: usize,
    pub difficulty: Difficulty,
}

impl Game {
    /// Create a new game with a random secret code for the given difficulty
    pub fn new(difficulty: Difficulty) -> Self {
        let mut rng = rand::thread_rng();
        let colors = difficulty.colors();
        let secret_code: Vec<char> = (0..difficulty.code_length)
            .map(|_| *colors.choose(&mut rng).unwrap())
            .collect();

        Game {
            secret_code,
            attempts: 0,
            difficulty,
        }
    }

    /// Validate a guess string against the current difficulty
    pub fn validate_guess(&self, guess: &str) -> Result<Vec<char>, String> {
        if guess.len() != self.difficulty.code_length {
            return Err(format!(
                "Invalid length! Please enter exactly {} colors.",
                self.difficulty.code_length
            ));
        }

        let guess_upper: Vec<char> = guess.to_uppercase().chars().collect();
        let allowed = self.difficulty.colors();

        for &ch in &guess_upper {
            if !allowed.contains(&ch) {
                return Err(format!(
                    "Invalid color '{}'. Use only: {}",
                    ch,
                    allowed.iter().collect::<String>()
                ));
            }
        }

        Ok(guess_upper)
    }

    /// Calculate feedback for a guess
    pub fn get_feedback(&self, guess: &[char]) -> Feedback {
        let mut exact_matches = 0;
        let mut secret_remaining = Vec::new();
        let mut guess_remaining = Vec::new();

        // First pass: find exact matches
        for i in 0..self.secret_code.len() {
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

    /// Register a guess attempt and return its feedback plus whether it won.
    pub fn submit_guess(&mut self, guess: &[char]) -> (Feedback, bool) {
        self.attempts += 1;
        let feedback = self.get_feedback(guess);
        let won = feedback.exact_matches == self.difficulty.code_length;
        (feedback, won)
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
            difficulty: Difficulty::CLASSIC,
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
            difficulty: Difficulty::CLASSIC,
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
            difficulty: Difficulty::CLASSIC,
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
            difficulty: Difficulty::CLASSIC,
        };
        let feedback = game.get_feedback(&['R', 'B', 'Y', 'M']);
        assert_eq!(feedback.exact_matches, 1); // R in position 0
        assert_eq!(feedback.color_matches, 2); // B and Y in wrong positions
    }

    #[test]
    fn test_validate_guess_valid() {
        let game = Game::new(Difficulty::CLASSIC);
        assert!(game.validate_guess("RGYB").is_ok());
        assert!(game.validate_guess("rgyb").is_ok()); // case insensitive
    }

    #[test]
    fn test_validate_guess_invalid_length() {
        let game = Game::new(Difficulty::CLASSIC);
        assert!(game.validate_guess("RGB").is_err());
        assert!(game.validate_guess("RGBYY").is_err());
    }

    #[test]
    fn test_validate_guess_invalid_color() {
        let game = Game::new(Difficulty::CLASSIC);
        assert!(game.validate_guess("RGBX").is_err());
    }

    // --- Difficulty-specific tests ---

    #[test]
    fn test_easy_rejects_colors_outside_palette() {
        // Easy uses only the first 4 colors: R, G, B, Y
        let game = Game::new(Difficulty::EASY);
        assert!(game.validate_guess("RGBY").is_ok());
        assert!(game.validate_guess("RGBM").is_err()); // M not in Easy palette
        assert!(game.validate_guess("RGBC").is_err()); // C not in Easy palette
    }

    #[test]
    fn test_easy_feedback_four_slots() {
        let game = Game {
            secret_code: vec!['R', 'G', 'B', 'Y'],
            attempts: 0,
            difficulty: Difficulty::EASY,
        };
        let feedback = game.get_feedback(&['G', 'R', 'B', 'Y']);
        assert_eq!(feedback.exact_matches, 2); // B, Y in place
        assert_eq!(feedback.color_matches, 2); // R, G swapped
    }

    #[test]
    fn test_hard_requires_five_slots() {
        let game = Game::new(Difficulty::HARD);
        assert!(game.validate_guess("RGBY").is_err()); // too short for Hard
        assert!(game.validate_guess("RGBYM").is_ok()); // 5 colors
    }

    #[test]
    fn test_hard_feedback_five_slots() {
        let game = Game {
            secret_code: vec!['R', 'G', 'B', 'Y', 'M'],
            attempts: 0,
            difficulty: Difficulty::HARD,
        };
        // All five exact
        let all = game.get_feedback(&['R', 'G', 'B', 'Y', 'M']);
        assert_eq!(all.exact_matches, 5);
        assert_eq!(all.color_matches, 0);

        // One exact (R), two color matches (G, B shifted), two absent (C, C)
        let mixed = game.get_feedback(&['R', 'B', 'G', 'C', 'C']);
        assert_eq!(mixed.exact_matches, 1); // R in position 0
        assert_eq!(mixed.color_matches, 2); // G and B present, wrong spot
    }

    #[test]
    fn test_difficulty_color_palettes() {
        assert_eq!(Difficulty::EASY.colors(), &['R', 'G', 'B', 'Y']);
        assert_eq!(Difficulty::CLASSIC.colors(), &['R', 'G', 'B', 'Y', 'M', 'C']);
        assert_eq!(Difficulty::HARD.colors(), &['R', 'G', 'B', 'Y', 'M', 'C']);
    }
}
