#[macro_use] extern crate itertools;

use itertools::Itertools;
use std::io::{self, Write, BufRead};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
struct Item {
    name: String,
    cost: i32,
    damage: i32,
    armor: i32
}

impl Item {
    pub fn new(name: &str, cost: i32, damage: i32, armor: i32) -> Item {
        Item {
            name: name.to_string(),
            cost: cost,
            damage: damage,
            armor: armor
        }
    }
}

fn main() {
    let weapons = vec![
        Item::new("Dagger", 8, 4, 0),
        Item::new("Shortsword", 10, 5, 0),
        Item::new("Warhammer", 25, 6, 0),
        Item::new("Longsword", 40, 7, 0),
        Item::new("Greataxe", 74, 8, 0)
    ];

    let armor = vec![
        Item::new("(None)", 0, 0, 0),
        Item::new("Leather", 13, 0, 1),
        Item::new("Chainmail", 31, 0, 2),
        Item::new("Splintmail", 53, 0, 3),
        Item::new("Bandedmail", 75, 0, 4),
        Item::new("Platemail", 102, 0, 5)
    ];

    let rings = vec![
        Item::new("(None)", 0, 0, 0),
        Item::new("Damage +1", 25, 1, 0),
        Item::new("Damage +2", 50, 2, 0),
        Item::new("Damage +3", 100, 3, 0),
        Item::new("Defense +1", 20, 0, 1),
        Item::new("Defense +2", 40, 0, 2),
        Item::new("Defense +3", 80, 0, 3)
    ];

    let stdin = io::stdin();

    println!("What's the boss HP?");
    print!("> ");
    io::stdout().flush().unwrap();
    let boss_hp = i32::from_str(&stdin.lock().lines().next().unwrap().unwrap()).unwrap();

    println!("What's the boss attack?");
    print!("> ");
    io::stdout().flush().unwrap();
    let boss_atk = i32::from_str(&stdin.lock().lines().next().unwrap().unwrap()).unwrap();

    println!("What's the boss defense?");
    print!("> ");
    io::stdout().flush().unwrap();
    let boss_def = i32::from_str(&stdin.lock().lines().next().unwrap().unwrap()).unwrap();

    let prod = iproduct!(weapons.iter(), armor.iter(), rings.iter(), rings.iter()).filter_map(|(ref weapon, ref armor, ref ring1, ref ring2)| {
        if ring1 == ring2 {
            return None;
        }

        let items = vec![weapon, armor, ring1, ring2];

        let dmg = items.iter().map(|x| x.damage).fold(0, |a, b| a + b);
        let def = items.iter().map(|x| x.armor).fold(0, |a, b| a + b);
        let cost = items.iter().map(|x| x.cost).fold(0, |a, b| a + b);

        let mut player_hp = 100i32;
        let mut boss_hp = 100i32;

        let mut win = true;

        loop {
            let player_dmg = dmg - boss_def;
            boss_hp -= player_dmg as i32;

            if boss_hp <= 0 {
                win = true;
                break;
            }

            let boss_dmg = boss_atk - def;
            player_hp -= boss_dmg as i32;

            if player_hp <= 0 {
                win = false;
                break;
            }
        }

        if win {
            None
        } else {
            Some(cost)
        }
    });

    println!(" - Henry can lose to the boss using {} gold -", prod.max().unwrap());
}
