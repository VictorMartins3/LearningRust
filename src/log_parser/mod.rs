use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use crate::game::Game;
use regex::Regex;

pub fn read_log_file(file_path: &str) -> Result<Vec<String>, Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

pub fn parse_log(lines: Vec<String>) -> Vec<Game> {
    let mut games = Vec::new();
    let mut current_game = Game::new();
    let mut count_init = 0;

    let re = Regex::new(r"Kill: \d+ \d+ \d+: (.+?) killed (.+?) by (\w+)").unwrap();

    for (_index, line) in lines.iter().enumerate() {
        if line.contains("InitGame") {
            if count_init != 0 {
                games.push(current_game);
                current_game = Game::new();
            }
            count_init += 1;
        } else if line.contains("ClientUserinfoChanged") {
            if let Some(name) = line.split("n\\").nth(1).and_then(|s| s.split('\\').next()) {
                current_game.add_player(name.to_string());
            }
        } else if line.contains("Kill") {
            if let Some(captures) = re.captures(line) {
                let killer = captures.get(1).map_or("", |m| m.as_str());
                let victim = captures.get(2).map_or("", |m| m.as_str());


                let means_of_death = captures.get(3).map_or("", |m| m.as_str());
                current_game.add_kill(killer, victim, means_of_death);
            }
        }
    }

    if current_game.total_kills > 0 || !current_game.players.is_empty() {
        games.push(current_game);
    }

    games
}
