extern crate regex;
extern crate permutohedron;

use std::io::{self, BufRead};
use std::string::ToString;
use std::str::FromStr;
use std::collections::HashMap;
use regex::Regex;
use permutohedron::Heap;

fn main() {
    println!("Accepting lines from stdin, Ctrl-D, Enter to stop");
    let stdin = io::stdin();

    let regex = Regex::new(r"(\w+) to (\w+) = (\d+)").unwrap();

    let mut cities = Vec::new();
    let mut pairs = HashMap::new();

    for line_ in stdin.lock().lines() {
        let line = line_.unwrap();
        let line_str = &line;

        if line_str == "\x04" {
            break;
        }

        let caps = regex.captures(&line).unwrap();
        let (from, to, distance) = (String::from_str(&caps[1]).unwrap(), String::from_str(&caps[2]).unwrap(), u32::from_str(&caps[3]).unwrap());

        pairs.insert((from.clone(), to.clone()), distance);
        pairs.insert((to.clone(), from.clone()), distance);

        if !cities.contains(&from) {
            cities.push(from);
        }

        if !cities.contains(&to) {
            cities.push(to);
        }
    }

    let heap = Heap::new(&mut cities);
    let mut paths = heap.map(|path| {
        let length = path.windows(2).map(|pair| {
            let a = String::from_str(&*pair[0]).unwrap();
            let b = String::from_str(&*pair[1]).unwrap();
            pairs.get(&(a, b)).unwrap()
        }).fold(0, |a, b| a + b);

        (length, path)
    }).collect::<Vec<_>>();
    paths.sort_by(|&(la, _), &(lb, _)| la.cmp(&lb));

    println!(" - Shortest path length is {} -", match paths.first().unwrap() {
        &(length, _) => length
    });

    println!(" - Longest path length is {} -", match paths.last().unwrap() {
        &(length, _) => length
    });
}
