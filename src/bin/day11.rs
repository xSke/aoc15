extern crate regex;

use std::io::{self, Write, BufRead};
use regex::Regex;


fn from_int(num: u64, alphabet: &Vec<char>) -> String {
    if num < alphabet.len() as u64 {
        return format!("{}", alphabet[num as usize]);
    } else {
        return format!("{}{}", from_int(num / alphabet.len() as u64, &alphabet), alphabet[(num % alphabet.len() as u64) as usize]);
    }
}

// WARNING
// This function is super buggy with the letter "a", however this does not interfere with my specific puzzle input so it still works
fn to_int(val: &str, alphabet: &Vec<char>) -> u64 {
    let base = alphabet.len();
    let mut num = 0u64;

    for i in 0..val.len() {
        let c = val.chars().nth(val.len() - i - 1).unwrap();
        num += base.pow(i as u32) as u64 * alphabet.iter().position(|x| *x == c).unwrap() as u64;
    }
    num
}

fn matches(pw: &str, regex1: &Regex, regex2: &Regex) -> bool {
    if pw.contains('i') || pw.contains('o') || pw.contains('l') {
        return false;
    }

    if regex1.find(pw).is_none() {
        return false;
    }

    if regex2.find_iter(pw).count() < 2 {
        return false;
    }

    return true;
}

fn main() {
    println!("What's the input?");
    print!("> ");
    io::stdout().flush().unwrap();

    let stdin = io::stdin();
    let pw = stdin.lock().lines().next().unwrap().unwrap();

    let alphabet = "abcdefghijklmnopqrstuvwxyz".chars().collect::<Vec<_>>();
    let regex1 = Regex::new("(abc|bcd|cde|def|efg|fgh|pqr|qrs|rst|stu|tuv|uvw|vwx|wxy|xyz)").unwrap();
    let regex2 = Regex::new("(aa|bb|cc|dd|ee|ff|gg|hh|ii|jj|kk|ll|mm|nn|oo|pp|qq|rr|ss|tt|uu|vv|ww|xx|yy|zz)").unwrap();

    let mut pw_int = to_int(&pw, &alphabet);
    while !matches(&from_int(pw_int, &alphabet), &regex1, &regex2) {
        pw_int += 1;
    }
    println!(" - The next password match is {} -", from_int(pw_int, &alphabet));

    pw_int += 1;
    while !matches(&from_int(pw_int, &alphabet), &regex1, &regex2) {
        pw_int += 1;
    }
    println!(" - The next password match is {} -", from_int(pw_int, &alphabet));
}
