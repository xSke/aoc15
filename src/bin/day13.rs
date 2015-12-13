extern crate regex;
extern crate permutohedron;

use std::io::{self, BufRead};
use std::str::FromStr;
use regex::Regex;
use std::collections::HashMap;
use permutohedron::Heap;

fn calc(people: &mut Vec<String>, pairs: &HashMap<(String, String), i32>) -> (Vec<String>, i32) {
    let heap = Heap::new(people);
    let mut paths = heap.map(|seatings| {
        let len = seatings.len() as isize;
        let res = seatings.iter().enumerate().map(|(i, x)| {
            let mut neighbor1 = (i as isize) - 1;
            if neighbor1 < 0 {neighbor1 += len}

            let mut neighbor2 = (i as isize) + 1;
            if neighbor2 == len {neighbor2 = 0}

            let a = String::from_str(&*x).unwrap();
            let b = String::from_str(&*seatings[neighbor1 as usize]).unwrap();
            let c = String::from_str(&*x).unwrap();
            let d = String::from_str(&*seatings[neighbor2 as usize]).unwrap();

            let mut units = 0i32;
            units += *pairs.get(&(a, b)).unwrap();
            units += *pairs.get(&(c, d)).unwrap();
            units
        }).fold(0, |a, b| a + b);
        (seatings, res)
    }).collect::<Vec<_>>();
    paths.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
    paths.first().unwrap().clone()
}

fn main() {
    let regex = Regex::new(r"(\w+) would (gain|lose) (\d+) happiness units by sitting next to (\w+).").unwrap();

    println!("Accepting lines from stdin, Ctrl-D, Enter to stop");
    let stdin = io::stdin();

    let mut people = vec![];
    let mut pairs = HashMap::new();

    for line_ in stdin.lock().lines() {
        let line = line_.unwrap();
        let line_str = &line;

        if line_str == "\x04" {
            break;
        }

        let caps = regex.captures(line_str).unwrap();
        let (a, b) = (String::from_str(&caps[1]).unwrap(), String::from_str(&caps[4]).unwrap());

        let mut amount = i32::from_str(&caps[3]).unwrap();
        if &caps[2] == "lose" {
            amount = -amount;
        }

        pairs.insert((a.clone(), b.clone()), amount);


        if !people.contains(&a) {
            people.push(a);
        }
        if !people.contains(&b) {
            people.push(b);
        }
    }

    println!(" - The optimal seating position has a total of {} happiness units -", match calc(&mut people, &pairs) {
        (_, amnt) => amnt
    });

    for person in &people {
        pairs.insert((person.clone(), "Me".to_string()), 0);
        pairs.insert(("Me".to_string(), person.clone()), 0);
    }
    people.push("Me".to_string());

    println!(" - The optimal seating position (with me included) has a total of {} happiness units -", match calc(&mut people, &pairs) {
        (_, amnt) => amnt
    });
}
