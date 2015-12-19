extern crate itertools;
extern crate regex;

use itertools::Itertools;
use regex::Regex;
use std::io::{self, BufRead, Write};

fn main() {
    println!("Accepting lines from stdin, empty line to stop");
    let stdin = io::stdin();

    let mut replacements = vec![];
    let regex = Regex::new(r"(\w+) => (\w+)").unwrap();

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let line_str = &line;

        if line_str.len() == 0 {
            break;
        }

        let caps = regex.captures(line_str).unwrap();
        replacements.push((caps[1].to_string(), caps[2].to_string()));
    }

    println!("What's the input?");
    print!("> ");
    io::stdout().flush().unwrap();

    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();

    replacements.sort_by(|a, b| a.0.cmp(&b.0));
    let mut outs = replacements.iter()
        .group_by(|x| &*x.0).collect::<Vec<_>>().into_iter()
        .flat_map(|(k, vs)| {
            vs.into_iter().flat_map(|v| {
                let indices = input.match_indices(&k).collect::<Vec<_>>();
                indices.iter().map(|&(pos, m)| {
                    format!("{}{}{}", &input[0..pos], v.1, &input[pos+m.len()..input.len()])
                }).collect::<Vec<_>>()
            }).collect::<Vec<_>>()
        }).collect::<Vec<_>>();
    outs.sort();
    outs.dedup();

    println!("{:?}", outs.len());
}
