extern crate regex;

use std::io::{self, BufRead};
use std::collections::HashMap;
use std::str::FromStr;
use regex::Regex;

#[derive(Debug)]
enum ReindeerState {
    Flying(u32),
    Resting(u32)
}

#[derive(Debug)]
struct Reindeer {
    state: ReindeerState,
    fly_time: u32,
    rest_time: u32,
    speed: u32,
    moved_distance: u32,
    points: u32
}

fn main() {
    println!("Accepting lines from stdin, Ctrl-D, Enter to stop");
    let stdin = io::stdin();

    let regex = Regex::new(r"(\w+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.").unwrap();
    let mut reindeers = HashMap::new();

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let line_str = &line;

        if line_str == "\x04" {
            break;
        }

        let cap = regex.captures(line_str).unwrap();

        let name = &cap[1];
        let speed = u32::from_str(&cap[2]).unwrap();
        let fly_time = u32::from_str(&cap[3]).unwrap();
        let rest_time = u32::from_str(&cap[4]).unwrap();

        let reindeer = Reindeer {
            state: ReindeerState::Flying(fly_time),
            fly_time: fly_time,
            rest_time: rest_time,
            speed: speed,
            moved_distance: 0,
            points: 0
        };

        reindeers.insert(name.to_string(), reindeer);
    }

    for _ in 0..2503 {
        for (_, reindeer) in reindeers.iter_mut() {
            reindeer.state = match reindeer.state {
                ReindeerState::Flying(remaining) => {
                    if remaining > 0 {
                        ReindeerState::Flying(remaining - 1)
                    } else {
                        ReindeerState::Resting(reindeer.rest_time - 1)
                    }
                },
                ReindeerState::Resting(remaining) => if remaining > 0 {
                    ReindeerState::Resting(remaining - 1)
                } else {
                    ReindeerState::Flying(reindeer.fly_time - 1)
                },
            };

            if let ReindeerState::Flying(_) = reindeer.state {
                reindeer.moved_distance += reindeer.speed;
            }
        }

        let lead = reindeers.iter().map(|(_, x)| x.moved_distance).max().unwrap();
        for reindeer in reindeers.iter_mut().map(|(_, x)| x).filter(|x| x.moved_distance == lead) {
            reindeer.points += 1;
        }
    }

    let mut sorted_distance = reindeers.iter().collect::<Vec<_>>();
    sorted_distance.sort_by(|&(_, a), &(_, b)| a.moved_distance.cmp(&b.moved_distance));

    let mut sorted_points = reindeers.iter().collect::<Vec<_>>();
    sorted_points.sort_by(|&(_, a), &(_, b)| a.points.cmp(&b.points));

    let &(fastest_name, fastest) = sorted_distance.last().unwrap();
    let &(best_name, best) = sorted_points.last().unwrap();
    println!(" - The fastest reindeer ({}) has traveled {} km -", fastest_name, fastest.moved_distance);
    println!(" - The reindeer with the most lead points ({}) has {} points -", best_name, best.points);
}
