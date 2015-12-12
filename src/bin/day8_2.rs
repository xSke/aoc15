use std::io::{self, BufRead};
use std::string::ToString;
use std::str::FromStr;

fn main() {
    println!("Accepting lines from stdin, Ctrl-D, Enter to stop");
    let stdin = io::stdin();

    let mut totaldelta = 0;
    for line_ in stdin.lock().lines() {
        let line = line_.unwrap();
        let line_str = &line;

        if line_str == "\x04" {
            break;
        }

        let escaped = format!("\"{}\"", line.chars().flat_map(|c| {
            match c {
                '"' => vec!['\\', '"'],
                '\\' => vec!['\\', '\\'],
                _ => vec![c]
            }
        }).collect::<String>());

        let lendelta = escaped.chars().count() - line.chars().count();
        totaldelta += lendelta;
    }

    println!(" - The total difference is {} -", totaldelta);
}
