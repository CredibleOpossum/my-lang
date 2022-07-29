mod instructions;
use instructions::Data;
use instructions::Instruction;

use std::env;

use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

use text_io::read;

use std::collections::HashMap;

use rand::prelude::*;

fn print_error(lines: &[String], index: usize, error: &str) {
    println!("----------------------------------------------------------------------------");
    let safe_minimum = (std::cmp::max(index as i32 - 5, 0)) as usize;
    for (offset, line) in lines[safe_minimum..index].iter().enumerate() {
        eprintln!("{: >8} | {}", safe_minimum + offset, line);
    }
    eprintln!(
        "{: >8} | {} <----------------- {}",
        index, lines[index], error
    );
    let safe_minimum = std::cmp::min(index as i32 + 1, lines.len() as i32) as usize;
    let safe_maximum = std::cmp::min(lines.len(), index + 6);
    for (offset, line) in lines[safe_minimum..safe_maximum].iter().enumerate() {
        eprintln!("{: >8} | {}", safe_minimum + offset, line);
    }
    println!("----------------------------------------------------------------------------");
    panic!("{:0>8}", error);
}

fn parser(file: &mut File) -> Vec<instructions::Instruction> {
    let buf = BufReader::new(file);

    let lines: Vec<String> = buf
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    let mut instructions: Vec<instructions::Instruction> = vec![instructions::Instruction::Start];

    let mut variables: HashMap<String, usize> = HashMap::new();
    let mut label_names: HashMap<String, usize> = HashMap::new();
    let mut labels = [0; 8192];

    let mut current_id = 0;
    let mut current_label_id = 0;

    for (index, raw_line) in lines.iter().enumerate() {
        let line = raw_line.trim();
        let params: Vec<&str> = line.split(' ').collect::<Vec<&str>>()[1..].to_vec();
        let instruction = line.split(' ').collect::<Vec<&str>>()[0];

        match instruction {
            "print" => instructions.push(
                match instructions::ins_print(params, &mut variables, &mut current_id, "") {
                    Ok(instruction) => instruction,
                    Err(why) => {
                        print_error(&lines, index, &why.to_string());
                        panic!("{}", why);
                    }
                },
            ),
            "printl" => instructions.push(
                match instructions::ins_print(params, &mut variables, &mut current_id, "\n") {
                    Ok(instruction) => instruction,
                    Err(why) => {
                        print_error(&lines, index, &why.to_string());
                        panic!("{}", why);
                    }
                },
            ),
            "set" => instructions.push(
                match instructions::ins_set(params, &mut variables, &mut current_id) {
                    Ok(instruction) => instruction,
                    Err(why) => {
                        print_error(&lines, index, &why.to_string());
                        panic!("{}", why);
                    }
                },
            ),
            "getnum" => instructions.push(instructions::ins_getnum(
                params,
                &mut variables,
                &mut current_id,
            )),
            "rand" => instructions.push(instructions::ins_rand(
                params,
                &mut variables,
                &mut current_id,
            )),
            "change" => instructions.push(instructions::ins_change(
                params,
                &mut variables,
                &mut current_id,
            )),
            "copy" => instructions.push(instructions::ins_copy(
                params,
                &mut variables,
                &mut current_id,
            )),
            "mul" => instructions.push(instructions::ins_mul(
                params,
                &mut variables,
                &mut current_id,
            )),
            "div" => instructions.push(instructions::ins_div(
                params,
                &mut variables,
                &mut current_id,
            )),
            "mod" => instructions.push(instructions::ins_mod(
                params,
                &mut variables,
                &mut current_id,
            )),
            "abs" => instructions.push(instructions::ins_abs(
                params,
                &mut variables,
                &mut current_id,
            )),
            "label" => {
                let id = instructions::get_or_set_id(
                    params[0].to_string(),
                    &mut label_names,
                    &mut current_label_id,
                );
                labels[id] = instructions.len() - 1;
            }
            "goto" => {
                let id = instructions::get_or_set_id(
                    params[0].to_string(),
                    &mut label_names,
                    &mut current_label_id,
                );
                instructions.push(Instruction::Goto(id))
            }
            "cmp" => instructions.push(instructions::ins_cmp(
                params,
                &mut variables,
                &mut label_names,
                &mut current_id,
                &mut current_label_id,
                false,
            )),
            "ncmp" => instructions.push(instructions::ins_cmp(
                params,
                &mut variables,
                &mut label_names,
                &mut current_id,
                &mut current_label_id,
                true,
            )),
            "ret" => instructions.push(Instruction::Ret),
            "end" => instructions.push(Instruction::End),
            "" => {},
            _ => {
                if !instruction.trim().is_empty() {
                    print_error(&lines, index, "unknown instruction")
                }

            }
        }
    }
    for instruction in &mut instructions {
        match *instruction {
            instructions::Instruction::Goto(id) => {
                *instruction = instructions::Instruction::Goto(labels[id]);
            }
            instructions::Instruction::Cmp(id, num, label_id, invert) => {
                *instruction = instructions::Instruction::Cmp(id, num, labels[label_id], invert)
            }
            _ => (),
        }
    }

    instructions
}

fn interpreter(instructions: Vec<Instruction>) {
    let mut rng = rand::thread_rng();
    let mut program_memory = [0; 8192];
    let mut jumps = [0; 100];
    let mut current_jump: i32 = 0;
    let mut instruction_pos = 0;
    let program_length = instructions.len();
    while instruction_pos < program_length {
        match &instructions[instruction_pos] {
            Instruction::Start => (),
            Instruction::Print(variable, string) => match variable {
                Some(pos) => print!(
                    "{}",
                    [program_memory[*pos].to_string(), string.to_string()].join("")
                ),
                None => print!("{}", string),
            },
            Instruction::Set(pos, num) => program_memory[*pos] = *num,
            Instruction::GetNum(pos) => {
                program_memory[*pos] = read!();
            }
            Instruction::Rand(pos) => program_memory[*pos] = rng.gen(),
            Instruction::Change(pos, data) => match data {
                Data::Address(address) => program_memory[*pos] += program_memory[*address],
                Data::Num(num) => program_memory[*pos] += *num,
            },
            Instruction::Copy(pos, pos2) => program_memory[*pos2] = program_memory[*pos],
            Instruction::Mul(pos, data) => match data {
                Data::Address(address) => program_memory[*pos] *= program_memory[*address],
                Data::Num(num) => program_memory[*pos] *= *num,
            },
            Instruction::Div(pos, data) => match data {
                Data::Address(address) => program_memory[*pos] /= program_memory[*address],
                Data::Num(num) => program_memory[*pos] /= *num,
            },
            Instruction::Mod(pos, data) => match data {
                Data::Address(address) => program_memory[*pos] %= program_memory[*address],
                Data::Num(num) => program_memory[*pos] %= *num,
            },
            Instruction::Abs(pos) => program_memory[*pos] = program_memory[*pos].abs(),
            Instruction::Goto(pos) => {
                jumps[current_jump as usize] = instruction_pos;
                current_jump += 1;
                current_jump = current_jump.rem_euclid(jumps.len() as i32);

                instruction_pos = *pos;
            }
            Instruction::Cmp(pos, data, jump_location, invert) => {
                let should_jump = match data {
                    Data::Address(address) => {
                        (program_memory[*pos] == program_memory[*address]) ^ invert
                    }
                    Data::Num(number) => (program_memory[*pos] == *number) ^ invert,
                };
                if should_jump {
                    jumps[current_jump as usize] = instruction_pos;
                    current_jump += 1;
                    current_jump = current_jump.rem_euclid(jumps.len() as i32);

                    instruction_pos = *jump_location;
                }
            }
            Instruction::Ret => {
                current_jump -= 1;
                current_jump = current_jump.rem_euclid(jumps.len() as i32);

                instruction_pos = jumps[current_jump as usize];
            }
            Instruction::End => {
                break;
            }
        }
        instruction_pos += 1;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => eprintln!("{}: no file supplied", args[0]),
        2 => {
            let path = Path::new(&args[1]);
            let instructions = match File::open(&path) {
                Err(why) => panic!("{}: could not open file: {}", args[0], why),
                Ok(mut file) => parser(&mut file),
            };
            interpreter(instructions);
        }
        _ => eprintln!("{}: too many arguments", args[0]),
    }
}
