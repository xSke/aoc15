use std::io::{self, BufRead};
use std::str::FromStr;

fn main() {
    let stdin = io::stdin();

    println!("Accepting lines from stdin, Ctrl-D, Enter to stop");

    let mut wrapping_paper_needed = 0;
    let mut ribbon_needed = 0;

    for line_ in stdin.lock().lines() {
        let line = line_.unwrap();

        if line == "\x04" {
            break;
        }

        // I'd use slice patterns here but they're experimental atm
        let segments: Vec<&str> = line.split("x").collect();
        if segments.len() != 3 {
            panic!("Must be three segments in present definition");
        }

        match (u32::from_str(segments[0]), u32::from_str(segments[1]), u32::from_str(segments[2])) {
            (Ok(l), Ok(w), Ok(h)) => {
                let surface_area = 2*l*w + 2*w*h + 2*h*l;

                let side_areas = [l*w, w*h, h*l];
                let smallest_size = side_areas.iter().min().unwrap();

                wrapping_paper_needed += surface_area + smallest_size;

                let perimeters = [l+l+w+w, w+w+h+h, h+h+l+l];
                let smallest_perimeter = perimeters.iter().min().unwrap();
                let volume = l*w*h;

                ribbon_needed += smallest_perimeter + volume;
            },
            _ => panic!("Could not parse present numbers")
        }
    }

    println!(" - The elves should order {} square feet of wrapping paper -", wrapping_paper_needed);
    println!(" - The elves should also order {} feet of ribbon -", ribbon_needed);
}
