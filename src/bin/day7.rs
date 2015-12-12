extern crate regex;

use std::io::{self, BufRead};
use std::collections::HashMap;
use std::str::FromStr;
use regex::Regex;

#[derive(Debug)]
enum Value {
    Wire(String),
    Number(u16)
}

#[derive(Debug)]
enum Wire {
    NumberInput(Value),
    AndInput(Value, Value),
    OrInput(Value, Value),
    LshiftInput(Value, Value),
    RshiftInput(Value, Value),
    NotInput(Value)
}

trait Input: Sized {
    fn calc(&self, registry: &HashMap<String, Wire>, cache: &mut HashMap<String, u16>) -> u16;
}

impl Input for Value {
    fn calc(&self, registry: &HashMap<String, Wire>, cache: &mut HashMap<String, u16>) -> u16 {
        match *self {
            Value::Wire(ref name) => {
                match cache.get(name) {
                    Some(res) => return *res,
                    None => {}
                }

                let result = registry.get(name).unwrap().calc(&registry, cache);
                cache.insert(name.clone(), result);
                result
            },
            Value::Number(val) => val
        }
    }
}

impl Input for Wire {
    fn calc(&self, registry: &HashMap<String, Wire>, cache: &mut HashMap<String, u16>) -> u16 {
        match *self {
            Wire::NumberInput(ref a) => a.calc(&registry, cache),
            Wire::AndInput(ref a, ref b) => a.calc(&registry, cache) & b.calc(&registry, cache),
            Wire::OrInput(ref a, ref b) => a.calc(&registry, cache) | b.calc(&registry, cache),
            Wire::LshiftInput(ref a, ref b) => a.calc(&registry, cache) << b.calc(&registry, cache),
            Wire::RshiftInput(ref a, ref b) => a.calc(&registry, cache) >> b.calc(&registry, cache),
            Wire::NotInput(ref a) => !a.calc(&registry, cache)
        }
    }
}

fn main() {
    let number_in = Regex::new(r"^([\w\d]+)([ ])-> ([\w\d]+)").unwrap();
    let and_gate = Regex::new(r"^([\w\d]+) AND ([\w\d]+) -> (\w+)").unwrap();
    let or_gate = Regex::new(r"^([\w\d]+) OR ([\w\d]+) -> (\w+)").unwrap();
    let lsh_gate = Regex::new(r"^([\w\d]+) LSHIFT ([\w\d]+) -> (\w+)").unwrap();
    let rsh_gate = Regex::new(r"^([\w\d]+) RSHIFT ([\w\d]+) -> (\w+)").unwrap();
    let not_gate = Regex::new(r"^NOT ([\w\d]+)([ ])-> ([\w\d]+)").unwrap();

    let mut wires: HashMap<String, Wire> = HashMap::new();
    let mut cache: HashMap<String, u16> = HashMap::new();

    println!("Accepting lines from stdin, Ctrl-D, Enter to stop");
    let stdin = io::stdin();
    for line_ in stdin.lock().lines() {
        let line = line_.unwrap();
        let line_str = &line;

        if line_str == "\x04" {
            break;
        }

        let cap1 = number_in.captures(line_str);
        let cap2 = and_gate.captures(line_str);
        let cap3 = or_gate.captures(line_str);
        let cap4 = lsh_gate.captures(line_str);
        let cap5 = rsh_gate.captures(line_str);
        let cap6 = not_gate.captures(line_str);

        let caps = vec![&cap1, &cap2, &cap3, &cap4, &cap5, &cap6].iter().flat_map(|x| x.iter()).next().unwrap();

        let a = match u16::from_str(&caps[1]) {
            Ok(num) => Value::Number(num),
            _ => Value::Wire(String::from_str(&caps[1]).unwrap())
        };

        let b = match u16::from_str(&caps[2]) {
            Ok(num) => Value::Number(num),
            _ => Value::Wire(String::from_str(&caps[2]).unwrap())
        };

        let key = String::from_str(&caps[3]).unwrap();

        let value = match (&cap1, &cap2, &cap3, &cap4, &cap5, &cap6) {
            (&Some(_), &None, &None, &None, &None, &None) => Some(Wire::NumberInput(a)),
            (&None, &Some(_), &None, &None, &None, &None) => Some(Wire::AndInput(a, b)),
            (&None, &None, &Some(_), &None, &None, &None) => Some(Wire::OrInput(a, b)),
            (&None, &None, &None, &Some(_), &None, &None) => Some(Wire::LshiftInput(a, b)),
            (&None, &None, &None, &None, &Some(_), &None) => Some(Wire::RshiftInput(a, b)),
            (&None, &None, &None, &None, &None, &Some(_)) => Some(Wire::NotInput(a)),
            _ => None
        };

        match value {
            Some(val) => {
                wires.insert(key, val);
            },
            _ => {}
        }
    }

    let a1 = wires.get("a").unwrap().calc(&wires, &mut cache);
    println!(" - a: {} -", a1);

    cache.clear();
    cache.insert(String::from_str("b").unwrap(), a1);

    let a2 = wires.get("a").unwrap().calc(&wires, &mut cache);
    println!(" - Overriding 'b' with value 'a' results in a: {} -", a2);
}
