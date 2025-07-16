use core::num;
use std::collections::HashMap;
use std::process::exit;
use std::io::{self, Write};
use crate::game::GameState;

mod game; 
mod bot; 

pub fn read_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap(); // stellt sicher, dass der Prompt sofort ausgegeben wird

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string() // entfernt \n und Leerzeichen am Anfang/Ende
}


fn start_game(bot: bool, print_mode: &PrintMode) -> (GameState, i32){
    let mut wordle = game::Wordle::new();

            if print_mode == &PrintMode::Normal || print_mode == &PrintMode::Detailed {
                wordle.print_board();
            }

    while wordle.game_state == game::GameState::PLAYING {

        let input = match bot {
            false => read_user_input("Guess: "),
            true => bot::get_best_word(&wordle)

        };


        if wordle.is_valid(&input) {
            wordle.input_word(&input);
            if print_mode == &PrintMode::Normal || print_mode == &PrintMode::Detailed {
                wordle.print_board();
            }
        }

        else {
            println!("Input is invalid..");
            println!("");
            wordle.print_board();
        }
    }
    return (wordle.game_state, wordle.guesses);

}
#[derive(PartialEq)]
enum PrintMode {
    Detailed, // alles was man printen kann
    Stats, // Nur die simulation stats
    Normal, // ganz normal das board und mit lost und so aber ohne sim stats
    Fast, // nur das end ergebnis der sim
    CompactSim,
}

pub fn start_bot(num_of_games: i32, print_mode: &PrintMode) {
    let mut wins = 0;
    let mut loses = 0;
    let mut games_played = 0;
    let mut guesses_overall = 0;




    for _ in 0..num_of_games {

        let result = start_game(true, &print_mode); // -> (GameState, u8)

        match result.0 {
            game::GameState::LOST => {

                if print_mode == &PrintMode::Normal || print_mode == &PrintMode::Detailed {
                    println!("💀 You lost this game.");
                }
                loses += 1;
            }
            game::GameState::WON => {
                if print_mode == &PrintMode::Normal || print_mode == &PrintMode::Detailed {
                    println!("🎉 You won this game!"); 
                }
                wins += 1;
            }
            _ => {
                eprintln!("❌ ERROR: Game ended unexpectedly.");
                exit(1);
            }
        }

        games_played += 1;
        if print_mode == &PrintMode::CompactSim {
            println!("Games played: {}", games_played);

        }
        guesses_overall += result.1;

        if print_mode == &PrintMode::Detailed || print_mode == &PrintMode::Stats{
            // Statistiken berechnen
            let avg_guesses = guesses_overall as f64 / games_played as f64;
            let win_rate = (wins as f64 / games_played as f64) * 100.0;
            let lose_rate = (loses as f64 / games_played as f64) * 100.0;

            println!("\n📊 === Simulation Statistics ===");
            println!("🕹️  Games played : {}", games_played);
            println!("✅ Wins          : {} ({:.1}%) {}", wins, win_rate, bar_graph(win_rate));
            println!("❌ Losses        : {} ({:.1}%) {}", loses, lose_rate, bar_graph(lose_rate));
            println!("🎯 Avg. guesses  : {:.2}", avg_guesses);
            println!("📈 Total guesses : {}", guesses_overall);
        }
    }
    if print_mode == &PrintMode::Detailed || print_mode == &PrintMode::Stats || print_mode == &PrintMode::Fast && print_mode == &PrintMode::CompactSim{
        // Statistiken berechnen
        let avg_guesses = guesses_overall as f64 / games_played as f64;
        let win_rate = (wins as f64 / games_played as f64) * 100.0;
        let lose_rate = (loses as f64 / games_played as f64) * 100.0;

        println!("\n📊 === Simulation Statistics ===");
        println!("🕹️  Games played : {}", games_played);
        println!("✅ Wins          : {} ({:.1}%) {}", wins, win_rate, bar_graph(win_rate));
        println!("❌ Losses        : {} ({:.1}%) {}", loses, lose_rate, bar_graph(lose_rate));
        println!("🎯 Avg. guesses  : {:.2}", avg_guesses);
        println!("📈 Total guesses : {}", guesses_overall);
    }

}

// Kleiner ASCII-Balken für visuelle Darstellung
fn bar_graph(percent: f64) -> String {
    let filled = (percent / 10.0).round() as usize;
    let bar: String = "█".repeat(filled) + &" ".repeat(10 - filled);
    format!("[{}]", bar)
}

fn play_normal_game() {
    let result = start_game(false, &PrintMode::Normal); // -> (GameState, u8)

    match result.0 {
        game::GameState::LOST => {
            println!("💀 You lost this game.");
            println!("");
        }
        game::GameState::WON => {
            println!("🎉 You won this game!"); 
            println!("");
        }
        _ => {
            eprintln!("❌ ERROR: Game ended unexpectedly.");
            exit(1);
        }
    }
}

fn main() {
    start_bot(10000, &PrintMode::Detailed);
    //play_normal_game();
}
