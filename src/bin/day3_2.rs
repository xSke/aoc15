use std::io::{self, BufRead};
use std::collections;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Default)]
struct Point {
    x: i32,
    y: i32
}

fn main() {
    println!("Accepting one line from stdin");

    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();

    let mut human_pos = Point {x: 0, y: 0};
    let mut robo_pos = Point {x: 0, y: 0};

    let mut houses = collections::HashSet::new();


    houses.insert(human_pos);
    for (i, c) in line.chars().enumerate() {
        let mut pos = if i % 2 == 0 {
            &mut human_pos
        } else {
            &mut robo_pos
        };

        match c {
            '^' => pos.y += 1,
            'v' => pos.y -= 1,
            '<' => pos.x -= 1,
            '>' => pos.x += 1,
            _ => continue
        }

        houses.insert(pos.clone());
    }

    println!(" - Santa needs to deliver presents to {} different houses -", houses.len());
}
