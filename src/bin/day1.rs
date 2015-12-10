use std::io::{self, Read};

fn main() {
    println!("Accepting input from stdin (unbuffered, so pipe input)");
    
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let mut counter = 0;
    for c in buffer.chars() {
        match c {
            '(' => counter += 1,
            ')' => counter -= 1,
            _ => {}
        }
    }

    println!(" - Santa is on floor {} -", counter)
}
