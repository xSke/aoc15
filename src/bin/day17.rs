use std::collections::HashMap;
use std::io::{self, BufRead};
use std::str::FromStr;

// http://stackoverflow.com/questions/4632322/finding-all-possible-combinations-of-numbers-to-reach-a-given-sum
fn recurse(numbers: &[u32], target: u32, partial: Vec<u32>) -> Vec<Vec<u32>> {
    let s = partial.iter().fold(0, |a, b| a + b);

    if s == target {
        return vec![partial];
    }

    if s > target {
        return vec![];
    }

    numbers.iter().enumerate().flat_map(|(i, n)| {
        let remaining = &numbers[i+1..];

        let mut next_partial = partial.clone();
        next_partial.extend(vec![n]);
        recurse(remaining, target, next_partial)
    }).collect::<Vec<_>>()
}

fn main() {
    println!("Accepting lines from stdin, Ctrl-D, Enter to stop");
    let stdin = io::stdin();

    let mut numbers = vec![];
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let line_str = &line;

        if line_str == "\x04" {
            break;
        }

        numbers.push(u32::from_str(line_str).unwrap());
    }

    let mut combinations = recurse(&numbers, 150, vec![]);
    println!(" - There are {} combinations of containers that fit 150 litres of eggnog -", combinations.len());

    combinations.sort_by(|a, b| a.len().cmp(&b.len()));

    let minimum = combinations.first().unwrap().len();
    let amount_minimum = combinations.iter().filter(|x| x.len() == minimum).count();
    println!(" - There are {} combinations of {} containers that fit 150 litres of eggnog -", minimum, amount_minimum);
}
