use std::io::{self, BufRead};

fn main() {
    println!("Accepting one line from stdin");

    let mut counter = 0;
    let mut basement_position = 0;

    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();

    for (i, c) in line.chars().enumerate() {
        match c {
            '(' => counter += 1,
            ')' => {
                counter -= 1;
                if counter == -1 && basement_position == 0 {
                    basement_position = i + 1;
                }
            },
            _ => {}
        }
    }

    println!(" - Santa is on floor {} -", counter);

    if basement_position > 0 {
        println!(" - Santa entered the basement at position {} -", basement_position);
    }
}
