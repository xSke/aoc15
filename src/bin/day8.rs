use std::io::{self, BufRead};

fn main() {
    println!("Accepting lines from stdin, Ctrl-D, Enter to stop");
    let stdin = io::stdin();

    let mut totaldelta = 0;
    for line_ in stdin.lock().lines() {
        let line = line_.unwrap();
        let line_str = &line;

        if line_str == "\x04" {
            break;
        }

        let mut unescaped = String::new();

        let mut list = line.chars().collect::<Vec<_>>();
        list.reverse();
        while let Some(next) = list.pop() {
            match next {
                '\\' => {
                    let next2 = list.pop().unwrap();
                    match next2 {
                        '\\' => unescaped.push('\\'),
                        '"' => unescaped.push('"'),
                        'x' => {
                            let ascii = format!("{}{}", list.pop().unwrap(), list.pop().unwrap());
                            unescaped.push(u8::from_str_radix(&ascii, 16).unwrap() as char);
                        }
                        _ => {
                            unescaped.push('\\');
                            unescaped.push(next2);
                        }
                    }
                    {}
                },
                _ => unescaped.push(next)
            };
        }
        let fnl = &unescaped[1..unescaped.len()-1];

        let lendelta = line.chars().count() - fnl.chars().count();
        totaldelta += lendelta;
    }

    println!(" - The total difference is {} -", totaldelta);
}
