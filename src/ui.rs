//! Terminal presentation for CipherMind — all rendering and input prompts.

use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
};
use std::io::{self, Write};

use crate::game::{Difficulty, Feedback};

/// Print a colored symbol based on the color character
pub fn print_colored_symbol(color_char: char) {
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
    )
    .unwrap();
}

/// Render a guess and its feedback line
pub fn show_guess_result(attempts: usize, guess: &[char], feedback: &Feedback) {
    print!("  Guess {}: ", attempts);
    for &color in guess {
        print_colored_symbol(color);
        print!(" ");
    }

    println!(
        "\n  → {} exact, {} color{}",
        feedback.exact_matches,
        feedback.color_matches,
        if feedback.color_matches != 1 { "s" } else { "" }
    );
}

/// Print encouraging hints based on feedback
pub fn print_hint(feedback: &Feedback, attempts: usize, max_attempts: usize) {
    let Feedback {
        exact_matches,
        color_matches,
    } = *feedback;

    let message = match (exact_matches, color_matches) {
        (0, 0) => "  💭 Hmm, try completely different colors!",
        (0, _) => "  💡 You have the right colors, just wrong positions!",
        (1, _) => "  🎯 Getting warmer! One's in the right spot!",
        (2, _) => "  🔥 Nice! Two are perfectly placed!",
        (3, _) => "  ⚡ So close! Just one more to go!",
        _ => "  🎲 Keep analyzing the patterns...",
    };

    println!("{}", message);

    if attempts >= max_attempts.saturating_sub(2) {
        println!("  ⏰ Running out of guesses!");
    }
}

/// Reveal the secret code
pub fn reveal_code(secret_code: &[char]) {
    print!("  The code was: ");
    for &color in secret_code {
        print_colored_symbol(color);
        print!(" ");
    }
    println!();
}

/// Prompt the player to choose a difficulty at the start of a game.
pub fn select_difficulty() -> Difficulty {
    println!("\n🎚️  Choose your difficulty:");
    for (i, d) in Difficulty::ALL.iter().enumerate() {
        println!(
            "  {}. {} — {} slots · {} colors · {} guesses",
            i + 1,
            d.name,
            d.code_length,
            d.num_colors,
            d.max_attempts
        );
    }

    loop {
        print!("\n👉 Enter 1-{} (default {}): ", Difficulty::ALL.len(), 2);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        // Empty input picks the Classic default (option 2).
        if input.is_empty() {
            return Difficulty::CLASSIC;
        }

        match input.parse::<usize>() {
            Ok(n) if (1..=Difficulty::ALL.len()).contains(&n) => return Difficulty::ALL[n - 1],
            _ => println!("  ❌ Please enter a number between 1 and {}.", Difficulty::ALL.len()),
        }
    }
}

/// Display the welcome banner for the chosen difficulty
pub fn print_welcome(difficulty: &Difficulty) {
    println!("\n╔════════════════════════════════════════════╗");
    println!("║          🧩 CIPHERMIND 🧩                 ║");
    println!("║   The Ultimate Code-Breaking Challenge    ║");
    println!("╚════════════════════════════════════════════╝\n");

    println!("🎮 How to Play  [{}]:", difficulty.name);
    println!(
        "  • I've created a secret {}-color code",
        difficulty.code_length
    );
    println!("  • Available colors: ",);
    print!("    ");
    for &color in difficulty.colors() {
        print_colored_symbol(color);
        print!(" = {} ", color);
    }
    println!(
        "\n  • You have {} guesses to crack it!",
        difficulty.max_attempts
    );
    println!("  • After each guess, I'll tell you:");
    println!("    - How many are EXACT (right color, right position)");
    println!("    - How many are COLOR matches (right color, wrong position)");
    let example: String = difficulty
        .colors()
        .iter()
        .cycle()
        .take(difficulty.code_length)
        .collect();
    println!(
        "\n💡 Example: Enter your guess as {} letters, like: {}\n",
        difficulty.code_length, example
    );
}

/// Ask if the player wants to play again
pub fn play_again() -> bool {
    print!("\n🔄 Play again? (y/n): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    matches!(input.trim().to_lowercase().as_str(), "y" | "yes")
}
