use std::io::{self, Read};

fn main() {
    println!("Accepting input from stdin, Ctrl-D, Enter to stop");

    let mut counter = 0;
    let mut basement_position = 0;
    // Using raw bytes here because Stdin::chars() is currently unstable, see issue #27802
    // Also we only need to look for bytes 40 and 41 so unicode doesn't matter and is ignored anyway
    for (i, c) in io::stdin().bytes().enumerate() {
        match c {
            // left paren = 40, right paren = 41
            Ok(40u8) => counter += 1,
            Ok(41u8) => {
                counter -= 1;
                if counter == -1 && basement_position == 0 {
                    basement_position = i + 1;
                }
            },
            // EOF
            Ok(4u8) => break,
            _ => println!("{}", c.unwrap())
        }
    }

    println!(" - Santa is on floor {} -", counter);

    if basement_position > 0 {
        println!(" - Santa entered the basement at position {} -", basement_position);
    }
}
