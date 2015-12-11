extern crate regex;

use std::io::{self, BufRead};
use regex::Regex;

fn main() {
    let vowel_re = Regex::new(r"[aeiou]").unwrap();
    let consecutive_re = Regex::new(r"(aa|bb|cc|dd|ee|ff|gg|hh|ii|jj|kk|ll|mm|nn|oo|pp|qq|rr|ss|tt|uu|vv|ww|xx|yy|zz)").unwrap();
    let disallowed_re = Regex::new(r"(ab|cd|pq|xy)").unwrap();

    println!("Accepting lines from stdin, Ctrl-D, Enter to stop");

    let mut nice_count = 0;

    let stdin = io::stdin();
    for line_ in stdin.lock().lines() {
        let line = line_.unwrap();

        if line == "\x04" {
            break;
        }
        
        let line_str = &line;

        let naughty = if vowel_re.find_iter(line_str).count() < 3 {
            true
        } else if !consecutive_re.is_match(line_str) {
            true
        } else if disallowed_re.is_match(line_str) {
            true
        } else {
            false
        };

        if !naughty {
            nice_count += 1;
        }
    }

    println!(" - Santa's text file has {} nice strings -", nice_count);
}
