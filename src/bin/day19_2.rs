extern crate itertools;
extern crate regex;

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

    println!("What's the output?");
    print!("> ");
    io::stdout().flush().unwrap();

    let stdin = io::stdin();
    let output = stdin.lock().lines().next().unwrap().unwrap();

    let mut o = output.clone();
    let mut i = 0;
    loop {
        for &(ref k, ref v) in &replacements {
            if o.matches(v).count() > 0 {
                let m = o.match_indices(v).next().unwrap().0;
                o = format!("{}{}{}", &o[0..m], k, &o[m+v.len()..o.len()]);
                i += 1;
            }
        }
        if o == "e" {
            println!(" - It takes {} replacements to reach the target molecule -", i);
            break;
        }
    }
}
