use std::io::{self, BufRead};
use std::collections;

fn main() {
    println!("Accepting one line from stdin");

    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();

    let mut x = 0;
    let mut y = 0;

    let mut houses = collections::HashSet::new();

    houses.insert((x, y));
    for c in line.chars() {
        match c {
            '^' => y += 1,
            'v' => y -= 1,
            '<' => x -= 1,
            '>' => x += 1,
            _ => continue
        }

        houses.insert((x, y));
    }

    println!(" - Santa needs to deliver presents to {} different houses -", houses.len());
}
