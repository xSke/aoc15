extern crate bit_vec;
extern crate regex;
use bit_vec::BitVec;
use std::io::{self, BufRead};
use regex::Regex;
use std::str::FromStr;

fn main() {
    let mut lights = BitVec::from_elem(1000000, false);

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
                        match &caps[1] {
                            "turn off" => lights.set(y*1000+x, false),
                            "turn on" => lights.set(y*1000+x, true),
                            "toggle" => {
                                let old = lights[y*1000+x];
                                lights.set(y*1000+x, !old);
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

    println!(" - There are {} lights -", lights.iter().filter(|x| x.clone()).count());
}
