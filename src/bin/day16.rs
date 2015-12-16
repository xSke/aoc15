extern crate regex;

use regex::Regex;

use std::collections::HashMap;
use std::io::{self, BufRead};
use std::str::FromStr;

fn main() {
    println!("Accepting lines from stdin, Ctrl-D, Enter to stop");
    let stdin = io::stdin();

    let regex = Regex::new(r"Sue \d+: (\w+): (\d+), (\w+): (\d+), (\w+): (\d+)").unwrap();

    let mut sues = Vec::new();

    let mut input = HashMap::new();
    input.insert("children".to_string(), 3);
    input.insert("cats".to_string(), 7);
    input.insert("samoyeds".to_string(), 2);
    input.insert("pomeranians".to_string(), 3);
    input.insert("akitas".to_string(), 0);
    input.insert("vizslas".to_string(), 0);
    input.insert("goldfish".to_string(), 5);
    input.insert("trees".to_string(), 3);
    input.insert("cars".to_string(), 2);
    input.insert("perfumes".to_string(), 1);

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let line_str = &line;

        if line_str == "\x04" {
            break;
        }

        let caps = regex.captures(line_str).unwrap();

        let (prop1, prop1_val) = (&caps[1], u32::from_str(&caps[2]).unwrap());
        let (prop2, prop2_val) = (&caps[3], u32::from_str(&caps[4]).unwrap());
        let (prop3, prop3_val) = (&caps[5], u32::from_str(&caps[6]).unwrap());

        let mut sue = HashMap::new();
        sue.insert(prop1.to_string(), prop1_val);
        sue.insert(prop2.to_string(), prop2_val);
        sue.insert(prop3.to_string(), prop3_val);

        sues.push(sue);
    }

    let sue_index = sues.iter().enumerate().filter(|&(_, sue)| {
        sue.iter().all(|(prop, val)| {
            let input_value = input.get(prop).expect(&format!("Could not find property {} in input", prop));
            match (*prop).trim() {
                "cats" | "trees" => val > input_value,
                "pomeranians" | "goldfish" => val < input_value,
                _ => val == input_value
            }
        })
    }).next().expect("Could not find matching Sue").0;

    println!(" - The Sue that got you the gift was Sue #{} -", sue_index + 1);
}
