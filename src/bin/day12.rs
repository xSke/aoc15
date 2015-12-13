extern crate regex;

use std::io::{self, BufRead};
use std::str::FromStr;
use regex::Regex;

fn main() {
    println!("Accepting lines from stdin, Ctrl-D, Enter to stop");
    let stdin = io::stdin();

    let regex = Regex::new(r"[-]?\d+").unwrap();

    let mut total = 0i64;
    for line_ in stdin.lock().lines() {
        let line = line_.unwrap();
        let line_str = &line;

        if line_str == "\x04" {
            break;
        }

        for (a, b) in regex.find_iter(&line) {
            total += i64::from_str(&line_str[a..b]).unwrap();
        }
    }

    println!(" - The total of all the numbers is {} -", total);
}
