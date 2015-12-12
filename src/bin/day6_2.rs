extern crate regex;
use std::io::{self, BufRead};
use regex::Regex;
use std::str::FromStr;

fn main() {
    let mut lights: Vec<u8> = vec![0u8;1000000];

    let regex = Regex::new(r"(toggle|turn on|turn off) (\d+),(\d+) through (\d+),(\d+)").unwrap();

    println!("Accepting lines from stdin, Ctrl-D, Enter to stop");
    let stdin = io::stdin();
    for line_ in stdin.lock().lines() {
        let line = line_.unwrap();

        if line == "\x04" {
            break;
        }

        match regex.captures(&line) {
            Some(caps) => {
                for x in usize::from_str(&caps[2]).unwrap()..usize::from_str(&caps[4]).unwrap()+1 {
                    for y in usize::from_str(&caps[3]).unwrap()..usize::from_str(&caps[5]).unwrap()+1 {
                        let i = y*1000+x;
                        match &caps[1] {
                            "turn off" => {
                                if lights[i] > 0 {
                                    lights[i] -= 1
                                }
                            },
                            "turn on" => lights[i] += 1,
                            "toggle" => {
                                lights[i] += 2
                            },
                            _ => {}
                        }
                    }
                }
            },
            None => {
                println!("Invalid pattern");
            }
        }
    }

    println!(" - The lights' total brightness is {} -", lights.iter().fold(0u64, |a, &b| {
        a + b as u64
    }));
}
