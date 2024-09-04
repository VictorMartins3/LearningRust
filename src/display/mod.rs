use crate::game::Game;

pub fn display_total_kills(games: &[Game]) {
    for (index, game) in games.iter().enumerate() {
        println!("game_{}:", index + 1);
        println!("  total_kills: {}", game.total_kills);
    }
}

pub fn display_players(games: &[Game]) {
    for (index, game) in games.iter().enumerate() {
        println!("game_{}:", index + 1);
        println!("  players: {:?}", game.players);
    }
}

pub fn display_kills(games: &[Game]) {
    for (index, game) in games.iter().enumerate() {
        println!("game_{}:", index + 1);
        println!("  kills:");
        let mut kills: Vec<(&String, &i32)> = game.kills.iter().collect();
        kills.sort_by(|a, b| b.1.cmp(a.1)); 
        for (player, kills) in kills {
            println!("    {}: {}", player, kills);
        }
    }
}

pub fn display_means_of_death(games: &[Game]) {
    for (index, game) in games.iter().enumerate() {
        println!("game_{}:", index + 1);
        println!("  means_of_death:");
        let mut means_of_death: Vec<(&String, &u32)> = game.means_of_death.iter().collect();
        means_of_death.sort_by(|a, b| b.1.cmp(a.1));  
        for (means, count) in means_of_death {
            println!("    {}: {}", means, count);
        }
    }
}

pub fn display_games_with_kills(games: &[Game]) {
    for (real_index, (filtered_index, game)) in games.iter().enumerate().filter(|(_, g)| g.total_kills > 0).enumerate() {
        println!("game_{} (real index {}):", filtered_index + 1, real_index + 1);
        println!("  total_kills: {}", game.total_kills);
        println!("  players: {:?}", game.players);
        println!("  kills:");
        let mut kills: Vec<(&String, &i32)> = game.kills.iter().collect();
        kills.sort_by(|a, b| b.1.cmp(a.1));  
        for (player, kills) in kills {
            println!("    {}: {}", player, kills);
        }
        println!("  means_of_death:");
        let mut means_of_death: Vec<(&String, &u32)> = game.means_of_death.iter().collect();
        means_of_death.sort_by(|a, b| b.1.cmp(a.1));  
        for (means, count) in means_of_death {
            println!("    {}: {}", means, count);
        }
    }
}

pub fn display_games_without_kills(games: &[Game]) {
    for (real_index, (filtered_index, game)) in games.iter().enumerate().filter(|(_, g)| g.total_kills == 0).enumerate() {
        println!("game_{} (real index {}):", filtered_index + 1, real_index + 1);
        println!("  total_kills: {}", game.total_kills);
        println!("  players: {:?}", game.players);
    }
}
