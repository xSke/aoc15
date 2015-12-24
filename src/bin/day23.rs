use std::collections::HashMap;
use std::io::{self, BufRead, Write};
use std::str::FromStr;

type Register = String;

#[derive(Debug)]
enum Instruction {
    Half(Register),
    Triple(Register),
    Increment(Register),
    Jump(i32),
    JumpIfEven(Register, i32),
    JumpIfOne(Register, i32)
}

fn sim(instructions: &Vec<Instruction>, registers: &mut HashMap<String, u64>) {
    let mut pc = 0i32;

    loop {
        let instr = instructions.get(pc as usize).unwrap();

        match *instr {
            Instruction::Half(ref r) => *registers.entry(r.to_string()).or_insert(0) /= 2,
            Instruction::Triple(ref r) => *registers.entry(r.to_string()).or_insert(0) *= 3,
            Instruction::Increment(ref r) => *registers.entry(r.to_string()).or_insert(0) += 1,
            Instruction::Jump(ref offset) => pc += offset - 1,
            Instruction::JumpIfEven(ref r, offset) => {
                if registers.get(r).unwrap_or(&0) % 2 == 0 {
                    pc += offset - 1
                }
            },
            Instruction::JumpIfOne(ref r, offset) => {
                if *registers.get(r).unwrap_or(&0) == 1 {
                    pc += offset - 1
                }
            }
        };

        pc += 1;
        if pc as usize >= instructions.len() {
            break;
        }
    }
}

fn main() {
    println!("Accepting lines from stdin, empty line to stop");
    let stdin = io::stdin();

    let mut instructions: Vec<Instruction> = vec![];

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let line_str = &line;

        if line_str == "\x04" {
            break;
        }

        let instruction = &line_str[0..3];
        let args = &line_str[4..];

        let instruction = match instruction {
            "hlf" => Instruction::Half(args.to_string()),
            "tpl" => Instruction::Triple(args.to_string()),
            "inc" => Instruction::Increment(args.to_string()),
            "jmp" => Instruction::Jump(i32::from_str(args).unwrap()),
            "jie" => Instruction::JumpIfEven(args[0..1].to_string(), i32::from_str(&args[3..]).unwrap()),
            "jio" => Instruction::JumpIfOne(args[0..1].to_string(), i32::from_str(&args[3..]).unwrap()),
            _ => Instruction::Half("a".to_string())
        };

        instructions.push(instruction);
    }

    let mut registers = HashMap::<String, u64>::new();
    sim(&instructions, &mut registers);

    println!(" - The value in register b is {} -", registers.get(&"b".to_string()).unwrap());

    registers.clear();
    registers.insert("a".to_string(), 1);
    sim(&instructions, &mut registers);

    println!(" - The value in register b is {} if a starts as 1 -", registers.get(&"b".to_string()).unwrap());
}
