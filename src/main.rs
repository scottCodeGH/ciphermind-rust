mod game;
mod ui;

use std::io::{self, Write};

use game::Game;

/// Main game loop
fn main() {
    loop {
        let difficulty = ui::select_difficulty();
        ui::print_welcome(&difficulty);

        let mut game = Game::new(difficulty);
        let mut won = false;

        // Main guessing loop
        while game.attempts < difficulty.max_attempts && !won {
            print!("\n🎯 Enter your guess (or 'quit' to exit): ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();

            // Check for quit
            if input.eq_ignore_ascii_case("quit") {
                println!("\n👋 Thanks for playing!");
                ui::reveal_code(&game.secret_code);
                return;
            }

            // Validate and process guess
            match game.validate_guess(input) {
                Ok(guess) => {
                    let (feedback, round_won) = game.submit_guess(&guess);
                    ui::show_guess_result(game.attempts, &guess, &feedback);

                    // Give encouraging hints while guesses remain
                    if !round_won && game.attempts < difficulty.max_attempts {
                        ui::print_hint(&feedback, game.attempts, difficulty.max_attempts);
                    }

                    won = round_won;
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
            println!(
                "You cracked the code in {} {}!",
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
            println!("You've used all {} attempts.", difficulty.max_attempts);
            ui::reveal_code(&game.secret_code);
            println!("\n🧠 Better luck next time! Each game is a new puzzle.");
        }
        println!("═══════════════════════════════════════════");

        // Ask to play again
        if !ui::play_again() {
            println!("\n👋 Thanks for playing CipherMind!");
            println!("Remember: Logic conquers all codes! 🧩\n");
            break;
        }
    }
}
