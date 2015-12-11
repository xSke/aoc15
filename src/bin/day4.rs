extern crate md5;

use std::io::{self, BufRead, Write};
use std::string::ToString;
use md5::Context;

fn main() {
    let stdin = io::stdin();

    println!("Do we find six zeroes? [y/N]");
    print!("> ");
    io::stdout().flush().unwrap();
    let six = stdin.lock().lines().next().unwrap().unwrap() == "y";

    println!("Finding {} zeroes.", if six {"six"} else {"five"});

    println!("What's the secret key?");
    print!("> ");
    io::stdout().flush().unwrap();

    let secret_key = stdin.lock().lines().next().unwrap().unwrap();

    let mut i = 0u64;
    loop {
        let mut context = Context::new();
        context.consume(secret_key.as_bytes());
        context.consume(i.to_string().as_bytes());
        let digest = context.compute();

        let matches = if six {
            digest[0] == 0 && digest[1] == 0 && digest[2] == 0
        } else {
            digest[0] == 0 && digest[1] == 0 && digest[2] >> 4 == 0
        };

        if matches {
            println!("\n - Santa's nonce is {} -", i);
            break;
        }

        if i % 50000 == 0 {
            print!(".");
            io::stdout().flush().unwrap();
        }

        i += 1;
    }
}
