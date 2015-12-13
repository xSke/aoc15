extern crate serde_json;

use std::io::{self, BufRead};
use std::str::FromStr;
use serde_json::Value;

fn recurse(value: &Value) -> i64 {
    match value {
        &Value::I64(i) => i,
        &Value::U64(u) => u as i64,
        &Value::F64(f) => f as i64,
        &Value::Array(ref arr) => arr.iter().map(|x| recurse(x)).fold(0, |a, b| a + b),
        &Value::Object(ref obj) => {
            if obj.iter().any(|(_, v)| {
                match v {
                    &Value::String(ref s) => s == "red",
                    _ => false
                }
            }) {
                0
            } else {
                obj.iter().map(|(_, v)| recurse(v)).fold(0, |a, b| a + b)
            }
        }
        _ => 0
    }
}

fn main() {
    println!("Accepting lines from stdin, Ctrl-D, Enter to stop");
    let stdin = io::stdin();

    let mut total = 0i64;
    for line_ in stdin.lock().lines() {
        let line = line_.unwrap();
        let line_str = &line;

        if line_str == "\x04" {
            break;
        }

        let data: Value = serde_json::from_str(line_str).unwrap();
        let count = recurse(&data);
        total += count;
    }

    println!(" - The total of all the numbers is {} -", total);
}
