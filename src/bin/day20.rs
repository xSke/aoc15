use std::io::{self, Write, BufRead};
use std::str::FromStr;

fn main() {
    println!("What's the input?");
    print!("> ");
    io::stdout().flush().unwrap();

    let stdin = io::stdin();
    let input_str = stdin.lock().lines().next().unwrap().unwrap();
    let input = usize::from_str(&input_str).unwrap();
    let input2 = input / 10;

    let mut houses = vec![0; input2 as usize];

    'outer: for num in 1..input2 {
        // Range::step_by is unstable :(

        let mut i = 0usize;
        while i < input2 {
            i += num;

            if i < input2 {
                houses[i] += num;
            }
        }
    }

    let out = houses.iter().enumerate().skip_while(|x| *x.1 < input2).next().unwrap().0;
    println!(" - The first house with {} or more presents is house #{} -", input, out);
}
