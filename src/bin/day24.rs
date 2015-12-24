use std::io::{self, BufRead};
use std::str::FromStr;

fn recurse(numbers: &[u64], target: u64, partial: Vec<u64>, limit: u64, curr: u64) -> Vec<Vec<u64>> {
    if curr >= limit {
        return vec![];
    }

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
        recurse(remaining, target, next_partial, limit, curr + 1)
    }).collect::<Vec<_>>()
}

fn main() {
    println!("Accepting lines from stdin, Ctrl-D, Enter to stop");
    let stdin = io::stdin();

    let mut presents = vec![];

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let line_str = &line;

        if line_str == "\x04" {
            break;
        }

        presents.push(u64::from_str(line_str).unwrap());
    }

    let run = |groups| {
        let total_weight = presents.iter().fold(0, |a, b| a + b);
        let group_weight = total_weight / groups;

        let first_groups = recurse(&presents, group_weight, vec![], (presents.len() / groups as usize) as u64, 0);
        let smallest = first_groups.iter().map(|x| x.len()).min().unwrap();

        let mut smallest_groups = first_groups.iter().filter(|x| x.len() == smallest).map(|x| (x, {
            x.iter().fold(1, |a, b| a * b)
        })).collect::<Vec<_>>();
        smallest_groups.sort_by(|a, b| a.1.cmp(&b.1));
        smallest_groups.first().unwrap().1
    };

    println!(" - The quantum entanglement of the optimal first group is {} -", run(3));
    println!(" - The quantum entanglement of the optimal first group (with a trunk) is {} -", run(4));
}
