extern crate regex;

use std::io::{self, BufRead};
use regex::Regex;

fn main() {
    println!("Accepting lines from stdin, Ctrl-D, Enter to stop");

    let mut nice_count = 0;

    let stdin = io::stdin();
    for line_ in stdin.lock().lines() {
        let line = line_.unwrap();

        if line == "\x04" {
            break;
        }

        let line_str = &line;

        let chars = line.chars().collect::<Vec<_>>();
        // This code is probably super slow
        let mut found_pair = false;

        chars.iter().fold(' ', |a, b| {
            if a != ' ' {
                let pair = format!("{}{}", a, b);
                // Using regex here because I can't think of any other way to find counts without overlapping
                let regex = Regex::new(&pair).unwrap();

                if regex.find_iter(line_str).count() >= 2 {
                    found_pair = true;
                }
            }
            b.clone()
        });

        let mut found_separated_pair = false;
        for (i, c) in chars.iter().enumerate() {
            if i <= chars.len() - 3 {
                if &chars[i + 2] == c {
                    found_separated_pair = true;
                }
            }
        }

        let naughty = !found_pair || !found_separated_pair;

        if !naughty {
            nice_count += 1;
        }
    }

    println!(" - Santa's text file has {} nice strings -", nice_count);
}
