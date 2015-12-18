extern crate bit_vec;

use bit_vec::BitVec;
use std::io::{self, BufRead};

fn xy2i(x: usize, y: usize) -> usize {
    return y * 100 + x;
}

fn tick(curr: &BitVec, out: &mut BitVec) {
    let neighbors = |x: usize, y: usize| {
        let mut neighbors = 0;
        for xx in -1..2 {
            for yy in -1..2 {
                if !(xx == 0 && yy == 0) {
                    match (x as isize + xx, y as isize + yy) {
                        (xxx @ 0...99, yyy @ 0...99) => {
                            if curr[xy2i(xxx as usize, yyy as usize)] {
                                neighbors += 1;
                            }
                        },
                        _ => {}
                    }
                }
            }
        }
        neighbors
    };

    for x in 0..100 {
        for y in 0..100 {
            let new = match (curr[xy2i(x, y)], neighbors(x, y)) {
                (true, 2...3) => true,
                (true, _) => false,
                (false, 3) => true,
                (false, _) => false
            };

            out.set(xy2i(x, y), new);
        }
    }
}

fn main() {
    println!("Accepting lines from stdin, Ctrl-D, Enter to stop");
    let stdin = io::stdin();

    // Double buffering!
    let mut lights_1 = BitVec::from_elem(100000, false);
    let mut lights_2 = BitVec::from_elem(100000, false);

    let mut i = 0;
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let line_str = &line;

        if line_str == "\x04" {
            break;
        }

        for light in line_str.chars().map(|x| match x {
            '#' => true,
             _ => false
        }) {
            lights_1.set(i, light);
            i += 1;
        }
    }

    let orig_input = lights_1.clone();

    for i in 0..100 {
        tick(&lights_1, &mut lights_2);
        std::mem::swap(&mut lights_1, &mut lights_2);
    }
    let amount = lights_1.iter().filter(|x| *x).count();

    println!(" - There are {:?} lights. -", amount);

    lights_1 = orig_input;
    for i in 0..100 {
        lights_1.set(xy2i(0, 0), true);
        lights_1.set(xy2i(99, 0), true);
        lights_1.set(xy2i(0, 99), true);
        lights_1.set(xy2i(99, 99), true);

        tick(&lights_1, &mut lights_2);
        std::mem::swap(&mut lights_1, &mut lights_2);

        lights_1.set(xy2i(0, 0), true);
        lights_1.set(xy2i(99, 0), true);
        lights_1.set(xy2i(0, 99), true);
        lights_1.set(xy2i(99, 99), true);
    }
    let amount = lights_1.iter().filter(|x| *x).count();
    println!(" - With the corner lights stuck, there are {:?} lights. -", amount);
}
