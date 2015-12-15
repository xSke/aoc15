extern crate regex;

use regex::Regex;

use std::cmp;
use std::collections::HashMap;
use std::io::{self, BufRead};
use std::str::FromStr;

fn splits(max: u32, amount: u32) -> Vec<Vec<u32>> {
    if amount == 1 {
        return vec![vec![max]];
    }

    (0..max+1).flat_map(|x| {
        let mut retval = splits(max - x, amount - 1);

        for combination in &mut retval {
            combination.push(x);
        }
        retval
    }).collect::<Vec<_>>()
}

struct Ingredient {
    properties: Vec<i32>,
    calories: u32
}

fn main() {
    println!("Accepting lines from stdin, Ctrl-D, Enter to stop");
    let stdin = io::stdin();

    let regex = Regex::new(r"(\w+): capacity ([-]?\d+), durability ([-]?\d+), flavor ([-]?\d+), texture ([-]?\d+), calories (\d+)").unwrap();

    let mut ingredients = HashMap::new();

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let line_str = &line;

        if line_str == "\x04" {
            break;
        }

        let caps = regex.captures(line_str).unwrap();
        let name = &caps[1];
        let capacity = i32::from_str(&caps[2]).unwrap();
        let durability = i32::from_str(&caps[3]).unwrap();
        let flavor = i32::from_str(&caps[4]).unwrap();
        let texture = i32::from_str(&caps[5]).unwrap();
        let calories = u32::from_str(&caps[6]).unwrap();

        ingredients.insert(name.to_string(), Ingredient {
            properties: vec![capacity, durability, flavor, texture],
            calories: calories
        });
    }

    let ingredients_ordered = ingredients.iter().collect::<Vec<_>>();
    let mut cookie_scores = splits(100, ingredients.len() as u32).iter().map(|split| {
        (
            (0..ingredients_ordered.first().unwrap().1.properties.len()).map(|prop_idx| {
                cmp::max(0, ingredients_ordered.iter().enumerate().map(|(ingredient_idx, &(_, ingredient))| {
                    split[ingredient_idx] as i32 * ingredient.properties[prop_idx]
                }).fold(0, |a, b| a + b))
            }).fold(1, |a, b| a * b),
            ingredients_ordered.iter().enumerate().map(|(ingredient_idx, &(_, ingredient))| {
                split[ingredient_idx] * ingredient.calories
            }).fold(0, |a, b| a + b)
        )
    }).collect::<Vec<_>>();
    cookie_scores.sort_by(|a, b| b.0.cmp(&a.0));

    println!(" - The total score of the best cookie is {} -", cookie_scores.first().unwrap().0);
    println!(" - The total score of the best cookie with exactly 500 calories is {} -", cookie_scores.iter().filter(|x| x.1 == 500).next().unwrap().0)
}
