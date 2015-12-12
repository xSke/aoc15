extern crate itertools;

use std::io::{self, Write, BufRead};
use std::str::FromStr;
use itertools::Itertools;

fn run(key: &str, iters: usize) -> String {
    (0..iters).fold(String::from_str(key).unwrap(), |last, i| {
        let groups = last.chars().peekable().batching(|mut it| {
            match it.next() {
                Some(first) => {
                    let mut vec = vec![first];
                    loop {
                        let next = {
                            let peek = it.peek();
                            match peek {
                                Some(x) => {
                                    if *x == first {
                                        true
                                    } else {
                                        false
                                    }
                                },
                                None => false
                            }
                        };
                        if next {
                            vec.push(it.next().unwrap())
                        } else {
                            break
                        }
                    }
                    Some(vec)
                },
                None => None
            }
        }).map(|x| {
            return format!("{}{}", x.len(), x.first().unwrap());
        }).collect::<String>();
        groups
    })
}

fn main() {
    println!("What's the input?");
    print!("> ");
    io::stdout().flush().unwrap();

    let stdin = io::stdin();
    let secret_key = stdin.lock().lines().next().unwrap().unwrap();

    println!(" - The length of the 40th iteration is {} -", run(&secret_key, 40).len());
    println!(" - The length of the 50th iteration is {} -", run(&secret_key, 50).len());
}
