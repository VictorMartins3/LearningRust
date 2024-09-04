use std::collections::HashMap;

pub const MEANS_OF_DEATH_LIST: [&str; 29] = [
    "MOD_UNKNOWN", "MOD_SHOTGUN", "MOD_GAUNTLET", "MOD_MACHINEGUN", "MOD_GRENADE", 
    "MOD_GRENADE_SPLASH", "MOD_ROCKET", "MOD_ROCKET_SPLASH", "MOD_PLASMA", "MOD_PLASMA_SPLASH",
    "MOD_RAILGUN", "MOD_LIGHTNING", "MOD_BFG", "MOD_BFG_SPLASH", "MOD_WATER", "MOD_SLIME", 
    "MOD_LAVA", "MOD_CRUSH", "MOD_TELEFRAG", "MOD_FALLING", "MOD_SUICIDE", "MOD_TARGET_LASER",
    "MOD_TRIGGER_HURT", "MOD_NAIL", "MOD_CHAINGUN", "MOD_PROXIMITY_MINE", "MOD_KAMIKAZE", 
    "MOD_JUICED", "MOD_GRAPPLE"
];

#[derive(Debug)]
pub struct Game {
    pub total_kills: u32,
    pub players: Vec<String>,
    pub kills: HashMap<String, i32>,
    pub means_of_death: HashMap<String, u32>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            total_kills: 0,
            players: Vec::new(),
            kills: HashMap::new(),
            means_of_death: HashMap::new(),
        }
    }

    pub fn add_player(&mut self, name: String) {
        if !self.players.contains(&name) {
            self.players.push(name.clone());
            self.kills.insert(name, 0);
        }
    }

    pub fn add_kill(&mut self, killer: &str, victim: &str, means_of_death: &str) {
        self.total_kills += 1;
        if MEANS_OF_DEATH_LIST.contains(&means_of_death) {
            *self.means_of_death.entry(means_of_death.to_string()).or_insert(0) += 1;
        }

        if killer != "<world>" && killer != victim {
            if let Some(kills) = self.kills.get_mut(killer) {
                *kills += 1;
            }
        } else if killer == "<world>" || killer == victim {
            if let Some(kills) = self.kills.get_mut(victim) {
                *kills -= 1;
            }
        }
    }
}
