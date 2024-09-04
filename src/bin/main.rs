use quake_log_parser::log_parser::{read_log_file, parse_log};
use quake_log_parser::report::generate_report;
use quake_log_parser::display::{display_total_kills, display_players, display_kills, display_means_of_death, display_games_with_kills, display_games_without_kills};
use std::io;

fn main() {
    let log_file_path = "src/log/qgames.log";  
    match read_log_file(log_file_path) {
        Ok(lines) => {
            let games = parse_log(lines);
            loop {
                println!("Select an option:");
                println!("1. Display full report");
                println!("2. Display total kills");
                println!("3. Display players");
                println!("4. Display kills");
                println!("5. Display means of death");
                println!("6. Display games with kills");
                println!("7. Display games without kills");
                println!("8. Exit");

                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");
                match input.trim() {
                    "1" => generate_report(&games),
                    "2" => display_total_kills(&games),
                    "3" => display_players(&games),
                    "4" => display_kills(&games),
                    "5" => display_means_of_death(&games),
                    "6" => display_games_with_kills(&games),
                    "7" => display_games_without_kills(&games),
                    "8" => break,
                    _ => println!("Invalid option, please try again."),
                }
            }
        },
        Err(err) => eprintln!("Error reading log file: {}", err),
    }
}
