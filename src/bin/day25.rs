use std::io::{self, Write, BufRead};
use std::str::FromStr;

fn main() {
    let stdin = io::stdin();

    println!("What's the row?");
    print!("> ");
    io::stdout().flush().unwrap();
    let row = i32::from_str(&stdin.lock().lines().next().unwrap().unwrap()).unwrap();

    println!("What's the column?");
    print!("> ");
    io::stdout().flush().unwrap();
    let col = i32::from_str(&stdin.lock().lines().next().unwrap().unwrap()).unwrap();

    let row_start = row + col - 1;

    let mut row_start_val = 1;
    let mut i = 0;
    loop {
        if i + 1 > row_start {
            break;
        }
        row_start_val += i;
        i += 1;
    }

    let code_number = row_start_val + col - 1;

    let mut code = 20151125u64;
    for _ in 1..code_number {
        code *= 252533;
        code %= 33554393;
    }

    println!(" - The code needed for the machine is {} -", code);
}
