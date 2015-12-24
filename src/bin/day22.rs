use std::io::{self, Write, BufRead};
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Spell {
    timer: u32,
    spell: SpellType
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum SpellType {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge
}

fn play(out: &mut Vec<u32>, hard_mode: bool, player_turn: bool, spells: Vec<Spell>, hp: i32, mana: i32, boss_hp: i32, boss_damage: i32, mana_spent: u32, round_count: u32) {
    if round_count > 22 {
        return;
    }

    let mut hp = hp;
    let mut boss_hp = boss_hp;
    let mut mana = mana;
    let mut armor = 0;

    if player_turn {
        if hard_mode {
            hp -= 1;
        }

        if hp <= 0 {
            return;
        }
    }

    let spells = spells.into_iter().map(|spell| {
        match spell.spell {
            SpellType::MagicMissile => boss_hp -= 4,
            SpellType::Drain => {
                boss_hp -= 2;
                hp += 2;
            },
            SpellType::Shield => {
                armor = 7;
            },
            SpellType::Poison => {
                boss_hp -= 3;
            },
            SpellType::Recharge => {
                mana += 101;
            }
        }
        let mut spell = spell.clone();
        spell.timer -= 1;
        spell
    }).filter(|x| x.timer > 0).collect::<Vec<_>>();

    if boss_hp <= 0 {
        out.push(mana_spent);
        return;
    }

    if hp <= 0 {
        return;
    }

    if player_turn {
        let mut try_next_cast = |spell: SpellType| {
            if !spells.iter().any(|x| x.spell == spell) {
                let cost = match spell {
                    SpellType::MagicMissile => 53,
                    SpellType::Drain => 73,
                    SpellType::Shield => 113,
                    SpellType::Poison => 173,
                    SpellType::Recharge => 229,
                } as u32;
                let mana = mana - cost as i32;

                let mut new_spells = spells.clone();
                new_spells.push(Spell {
                    timer: match spell {
                        SpellType::MagicMissile => 1,
                        SpellType::Drain => 1,
                        SpellType::Shield => 6,
                        SpellType::Poison => 6,
                        SpellType::Recharge => 5,
                    },
                    spell: spell
                });
                play(out, hard_mode, false, new_spells, hp, mana, boss_hp, boss_damage, mana_spent + cost, round_count + 1)
            }
        };

        if mana < 53 {
            return;
        }

        try_next_cast(SpellType::MagicMissile);
        try_next_cast(SpellType::Drain);
        try_next_cast(SpellType::Shield);
        try_next_cast(SpellType::Poison);
        try_next_cast(SpellType::Recharge);
    } else {
        let mut dmg = boss_damage - armor;
        if dmg <= 0 {
            dmg = 1;
        }
        hp -= dmg;
        play(out, hard_mode, true, spells, hp, mana, boss_hp, boss_damage, mana_spent, round_count + 1);
    }
}

fn main() {
    let stdin = io::stdin();

    println!("What's the boss HP?");
    print!("> ");
    io::stdout().flush().unwrap();
    let boss_hp = i32::from_str(&stdin.lock().lines().next().unwrap().unwrap()).unwrap();

    println!("What's the boss damage?");
    print!("> ");
    io::stdout().flush().unwrap();
    let boss_dmg = i32::from_str(&stdin.lock().lines().next().unwrap().unwrap()).unwrap();

    let mut out = vec![];
    play(&mut out, false, true, vec![], 50, 500, boss_hp, boss_dmg, 0, 0);

    println!(" - Henry can beat the boss using {} mana -", out.iter().min().unwrap());

    out.clear();
    play(&mut out, true, true, vec![], 50, 500, boss_hp, boss_dmg, 0, 0);

    println!(" - Henry can beat the boss using {} mana in hard mode -", out.iter().min().unwrap());

}
