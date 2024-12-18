use regex::Regex;

advent_of_code::solution!(17);

pub fn part_one(input: &str) -> Option<u128> {
    let re =
        Regex::new(r"Register A: (\d+)\nRegister B: (\d+)\nRegister C: (\d+)\n\nProgram: (.+)")
            .unwrap();

    let mut register_a: u128 = 0;
    let mut program: Vec<u32> = vec![];

    for (_, [reg_a, _, _, prog]) in re.captures_iter(input).map(|c| c.extract()) {
        register_a = reg_a.parse().unwrap();
        program = prog.split(",").map(|x| x.parse().unwrap()).collect();
    }

    let output: Vec<u32> = run_program(register_a, &program);

    println!(
        "{}",
        output
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",")
    );

    output
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse::<u128>()
        .ok()
}

pub fn part_two_bad(input: &str) -> Option<u128> {
    let re =
        Regex::new(r"Register A: (\d+)\nRegister B: (\d+)\nRegister C: (\d+)\n\nProgram: (.+)")
            .unwrap();

    let mut register_a: Box<u128> = Box::new(0);
    let mut register_b: Box<u128> = Box::new(0);
    let mut register_c: Box<u128> = Box::new(0);
    let mut program: Vec<u32> = vec![];
    // let mut output: Vec<u32> = vec![];
    for (_, [_, _, _, prog]) in re.captures_iter(input).map(|c| c.extract()) {
        program = prog.split(",").map(|x| x.parse().unwrap()).collect();
    }

    let mut pointer = program.len() - 2;
    let mut output_to_match = (program.len() - 1) as isize;
    while pointer < program.len() {
        let instruction = program[pointer];
        let operand = program[pointer + 1];

        let operand_value = match operand {
            4 => *register_a,
            5 => *register_b,
            6 => *register_c,
            _ => operand as u128,
        };

        // match instruction {
        //     0 => register_a = register_a / 2_u128.pow(operand_value as u32),
        //     1 => register_b = register_b ^ operand as u128,
        //     2 => register_b = operand_value % 8,
        //     3 => {
        //         if register_a != 0 {
        //             pointer = operand as usize;
        //             continue;
        //         }
        //     } //jumps to literal
        //     4 => register_b = register_b ^ register_c,
        //     5 => output.push((operand_value % 8) as u32),
        //     6 => register_b = register_a / 2_u128.pow(operand_value as u32),
        //     7 => register_c = register_a / 2_u128.pow(operand_value as u32),
        //     _ => panic!("NO OTHER INSTRUCTION EXIST"),
        // }

        match instruction {
            0 => register_a = Box::new(register_a.as_ref() * 2_u128.pow(operand_value as u32)),
            1 => register_b = Box::new(register_b.as_ref() ^ operand as u128),
            2 => register_b = Box::new((operand_value % 8) as u128),
            3 => {
                if output_to_match < 0 {
                    break;
                }
            } //jumps to literal
            4 => register_b = Box::new(register_b.as_ref() ^ register_c.as_ref()),
            5 => {
                let operand_value = match operand {
                    4 => &mut register_a,
                    5 => &mut register_b,
                    6 => &mut register_c,
                    _ => &mut Box::new(operand as u128),
                };
                let value_to_match = program[output_to_match as usize] as u128;
                let current_val = **operand_value % 8;
                let diff = value_to_match - current_val;
                *operand_value = Box::new(**operand_value + diff);
                output_to_match -= 1;
            }
            6 => register_a = Box::new(register_b.as_ref() * 2_u128.pow(operand_value as u32)),
            7 => register_a = Box::new(register_c.as_ref() * 2_u128.pow(operand_value as u32)),
            _ => panic!("NO OTHER INSTRUCTION EXIST"),
        }
        if pointer == 0 {
            pointer = program.len() - 2
        } else {
            pointer -= 2
        }
    }
    Some(*register_a.as_ref())
}

pub fn part_two(input: &str) -> Option<u128> {
    let re =
        Regex::new(r"Register A: (\d+)\nRegister B: (\d+)\nRegister C: (\d+)\n\nProgram: (.+)")
            .unwrap();

    let mut program: Vec<u32> = vec![];

    for (_, [_, _, _, prog]) in re.captures_iter(input).map(|c| c.extract()) {
        program = prog.split(",").map(|x| x.parse().unwrap()).collect();
    }

    Some(find_start_val(0, 1, &program))
}

pub fn find_start_val(current_val: u128, len_to_match: usize, program: &[u32]) -> u128 {
    if len_to_match > program.len() {
        return current_val;
    }

    let reg_a = current_val * 8;

    for i in 0..8 {
        let output = run_program(reg_a + i, program);
        if output == program[program.len() - len_to_match..program.len()] {
            let found_val = find_start_val(reg_a + i, len_to_match + 1, program);
            if found_val > 0 {
                return found_val;
            }
        }
    }

    0
}

pub fn run_program(register_a_val: u128, program: &[u32]) -> Vec<u32> {
    let mut register_a = register_a_val;
    let mut register_b = 0;
    let mut register_c = 0;
    let mut output = vec![];

    let mut pointer = 0;
    while pointer < program.len() {
        let instruction = program[pointer];
        let operand = program[pointer + 1];

        let operand_value = match operand {
            4 => register_a,
            5 => register_b,
            6 => register_c,
            _ => operand as u128,
        };

        match instruction {
            0 => register_a = register_a / 2_u128.pow(operand_value as u32),
            1 => register_b = register_b ^ operand as u128,
            2 => register_b = operand_value % 8,
            3 => {
                if register_a != 0 {
                    pointer = operand as usize;
                    continue;
                }
            } //jumps to literal
            4 => register_b = register_b ^ register_c,
            5 => output.push((operand_value % 8) as u32),
            6 => register_b = register_a / 2_u128.pow(operand_value as u32),
            7 => register_c = register_a / 2_u128.pow(operand_value as u32),
            _ => panic!("NO OTHER INSTRUCTION EXIST"),
        }
        pointer += 2
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4635635210))
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }
}
