use std::io::{self, Read};

fn main() {
    println!("Accepting input from stdin (unbuffered, so pipe input)");

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let mut counter = 0;
    let mut basement_position = 0;
    for (i, c) in buffer.chars().enumerate() {
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
