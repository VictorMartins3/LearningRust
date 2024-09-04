use crate::game::Game;

pub fn generate_report(games: &[Game]) {
    for (index, game) in games.iter().enumerate() {
        println!("game_{}:", index + 1);
        println!("  total_kills: {}", game.total_kills);
        println!("  players: {:?}", game.players);
        println!("  kills:");
        let mut kills: Vec<(&String, &i32)> = game.kills.iter().collect();
        kills.sort_by(|a, b| b.1.cmp(a.1));  // Ordena por número de kills, decrescente
        for (player, kills) in kills {
            println!("    {}: {}", player, kills);
        }
        println!("  means_of_death:");
        let mut means_of_death: Vec<(&String, &u32)> = game.means_of_death.iter().collect();
        means_of_death.sort_by(|a, b| b.1.cmp(a.1));  // Ordena por número de ocorrências, decrescente
        for (means, count) in means_of_death {
            println!("    {}: {}", means, count);
        }
    }
}
