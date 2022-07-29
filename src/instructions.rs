use std::collections::HashMap;
use std::error::Error;

#[derive(Debug, Clone, Copy)]
pub enum Data {
    // Type for memory location or number
    Address(usize),
    Num(i32),
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Start,
    Goto(usize),
    Set(usize, i32),
    GetNum(usize),
    Rand(usize),
    Cmp(usize, Data, usize, bool),
    Change(usize, Data),
    Copy(usize, usize),
    Mul(usize, Data),
    Div(usize, Data),
    Mod(usize, Data),
    Abs(usize),
    Print(Option<usize>, String),
    Ret,
    End,
}

fn is_number(string: &str) -> bool {
    string.parse::<i32>().is_ok()
}

fn get_text(command: Vec<&str>) -> String {
    let mut joined = command.join(" ");
    joined = joined[1..joined.len() - 1].to_string();
    joined
}

pub fn get_or_set_id(
    string: String,
    variables: &mut HashMap<String, usize>,
    current_id: &mut usize,
) -> usize {
    let temp_id;
    if let std::collections::hash_map::Entry::Vacant(e) = variables.entry(string.clone()) {
        temp_id = *current_id;
        e.insert(*current_id);
        *current_id += 1;
    } else {
        temp_id = variables[&string];
    }
    temp_id
}

pub fn ins_print(
    params: Vec<&str>,
    variables: &mut HashMap<String, usize>,
    current_id: &mut usize,
    suffix: &str,
) -> Result<Instruction, Box<dyn Error>> {
    return match params.get(0).ok_or("missing argument")?.starts_with('"') as u32
        + params[params.len() - 1].ends_with('"') as u32
    {
        2 => Ok(Instruction::Print(
            None,
            [get_text(params), suffix.to_string()].join(""),
        )),
        1 => panic!("Mismatched or unmatched quotation mark"),
        0 => {
            let id = get_or_set_id(params[0].to_string(), variables, current_id);
            Ok(Instruction::Print(Some(id), suffix.to_string()))
        }
        _ => Err("unknown error".into()),
    };
}

pub fn ins_set(
    params: Vec<&str>,
    variables: &mut HashMap<String, usize>,
    current_id: &mut usize,
) -> Result<Instruction, Box<dyn Error>> {
    Ok(Instruction::Set(
        get_or_set_id(params[0].to_string(), variables, current_id),
        params[1].parse()?,
    ))
}

pub fn ins_getnum(
    params: Vec<&str>,
    variables: &mut HashMap<String, usize>,
    current_id: &mut usize,
) -> Instruction {
    Instruction::GetNum(get_or_set_id(params[0].to_string(), variables, current_id))
}

pub fn ins_rand(
    params: Vec<&str>,
    variables: &mut HashMap<String, usize>,
    current_id: &mut usize,
) -> Instruction {
    Instruction::Rand(get_or_set_id(params[0].to_string(), variables, current_id))
}

pub fn ins_change(
    params: Vec<&str>,
    variables: &mut HashMap<String, usize>,
    current_id: &mut usize,
) -> Instruction {
    let data = if is_number(params[1]) {
        Data::Num(
            params[1]
                .parse()
                .expect("change command didn't contain vaild integer"),
        )
    } else {
        Data::Address(get_or_set_id(params[1].to_string(), variables, current_id))
    };
    Instruction::Change(
        get_or_set_id(params[0].to_string(), variables, current_id),
        data,
    )
}

pub fn ins_copy(
    params: Vec<&str>,
    variables: &mut HashMap<String, usize>,
    current_id: &mut usize,
) -> Instruction {
    Instruction::Copy(
        get_or_set_id(params[0].to_string(), variables, current_id),
        get_or_set_id(params[1].to_string(), variables, current_id),
    )
}

pub fn ins_mul(
    params: Vec<&str>,
    variables: &mut HashMap<String, usize>,
    current_id: &mut usize,
) -> Instruction {
    let data = if is_number(params[1]) {
        Data::Num(
            params[1]
                .parse()
                .expect("add command didn't contain vaild integer"),
        )
    } else {
        Data::Address(get_or_set_id(params[1].to_string(), variables, current_id))
    };
    Instruction::Mul(
        get_or_set_id(params[0].to_string(), variables, current_id),
        data,
    )
}

pub fn ins_div(
    params: Vec<&str>,
    variables: &mut HashMap<String, usize>,
    current_id: &mut usize,
) -> Instruction {
    let address_or_number = if is_number(params[1]) {
        Data::Num(
            params[1]
                .parse()
                .expect("add command didn't contain vaild integer"),
        )
    } else {
        Data::Address(get_or_set_id(params[1].to_string(), variables, current_id))
    };
    Instruction::Div(
        get_or_set_id(params[0].to_string(), variables, current_id),
        address_or_number,
    )
}

pub fn ins_mod(
    params: Vec<&str>,
    variables: &mut HashMap<String, usize>,
    current_id: &mut usize,
) -> Instruction {
    let data = if is_number(params[1]) {
        Data::Num(
            params[1]
                .parse()
                .expect("add command didn't contain vaild integer"),
        )
    } else {
        Data::Address(get_or_set_id(params[1].to_string(), variables, current_id))
    };
    Instruction::Mod(
        get_or_set_id(params[0].to_string(), variables, current_id),
        data,
    )
}

pub fn ins_abs(
    params: Vec<&str>,
    variables: &mut HashMap<String, usize>,
    current_id: &mut usize,
) -> Instruction {
    Instruction::Abs(get_or_set_id(params[0].to_string(), variables, current_id))
}

pub fn ins_cmp(
    params: Vec<&str>,
    variables: &mut HashMap<String, usize>,
    label_names: &mut HashMap<String, usize>,
    current_id: &mut usize,
    current_label_id: &mut usize,
    invert: bool,
) -> Instruction {
    let memory_id = get_or_set_id(params[0].to_string(), variables, current_id);
    let label_id = get_or_set_id(params[2].to_string(), label_names, current_label_id);
    let address_or_number = if is_number(params[1]) {
        Data::Num(params[1].parse().unwrap())
    } else {
        Data::Address(get_or_set_id(
            params[1].to_string(),
            variables,
            current_label_id,
        ))
    };
    Instruction::Cmp(memory_id, address_or_number, label_id, invert)
}
