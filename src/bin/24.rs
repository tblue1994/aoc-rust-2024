use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

use regex::Regex;

advent_of_code::solution!(24);

pub fn part_one(input: &str) -> Option<u64> {
    let mut known_values = HashMap::new();
    let mut gates = VecDeque::new();
    let re = Regex::new(r"(.+) (XOR|AND|OR) (.+) -> (.+)").unwrap();
    for line in input.lines() {
        if line.contains(":") {
            let mut splits = line.split(": ");
            let gate = splits.next().unwrap();
            let value = splits.next().unwrap().parse::<isize>().unwrap();
            known_values.insert(gate, value);
        } else if line.contains("->") {
            for (_, [gate1, op, gate2, dest]) in re.captures_iter(input).map(|c| c.extract()) {
                gates.push_back((gate1, op, gate2, dest));
            }
        }
    }

    while !gates.is_empty() {
        let gate = gates.pop_front().unwrap();
        if !known_values.contains_key(gate.0) || !known_values.contains_key(gate.2) {
            gates.push_back(gate);
            continue;
        }

        let val1 = known_values.get(gate.0).unwrap();
        let val2 = known_values.get(gate.2).unwrap();
        let found_val = match gate.1 {
            "OR" => val1 | val2,
            "AND" => val1 & val2,
            "XOR" => val1 ^ val2,
            _ => panic!("OP not recognized"),
        };
        known_values.insert(gate.3, found_val);
    }

    let mut z_values: Vec<(&&str, &isize)> = known_values
        .iter()
        .filter(|(k, _)| k.starts_with("z"))
        .collect();

    z_values.sort_by(|a, b| b.0.cmp(a.0));
    let mut binary_string: String = "".to_owned();
    for (_, x) in z_values {
        binary_string = binary_string + &x.to_string();
    }
    println!("{}", binary_string);
    let x = u64::from_str_radix(&binary_string, 2).unwrap();
    Some(x)
}

#[derive(Debug, Clone)]
pub struct Gate {
    op: String,
    gates: HashSet<String>,
    result: String,
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut known_values = HashMap::new();
    let mut gates = vec![];
    let re = Regex::new(r"(.+) (XOR|AND|OR) (.+) -> (.+)").unwrap();
    for line in input.lines() {
        if line.contains(":") {
            let mut splits = line.split(": ");
            let gate = splits.next().unwrap();
            let value = splits.next().unwrap().parse::<isize>().unwrap();
            known_values.insert(gate, value);
        } else if line.contains("->") {
            for (_, [gate1, op, gate2, dest]) in re.captures_iter(input).map(|c| c.extract()) {
                gates.push(Gate {
                    gates: HashSet::from([gate1.to_string(), gate2.to_string()]),
                    op: op.to_string(),
                    result: dest.to_string(),
                });
            }
        }
    }

    // go through and find x, y and z for w/e
    let mut current_carry = gates
        .iter()
        .find(|g| {
            g.op == "AND" && HashSet::from(["x00".to_string(), "y00".to_string()]) == g.gates
        })?
        .result
        .to_string();
    let mut swapped = BTreeSet::new();
    for dig in 1..known_values.len() / 2 - 1 {
        let dig_str = format!("{:02}", dig);
        check_gate(&dig_str, &current_carry, &gates, &mut swapped);
    }
    println!("{:?}", swapped);
    Some(swapped.len() as u32)
}

pub fn check_gate(
    digit_str: &str,
    _carry_gate: &str,
    gates: &[Gate],
    swapped: &mut BTreeSet<String>,
) {
    let x = "x".to_owned() + digit_str;
    let y = "y".to_owned() + digit_str;
    let z = "z".to_owned() + digit_str;
    let first_xor = find_gate(Some(&x), Some(&y), None, "XOR", gates).unwrap();
    if first_xor.result.starts_with("z") {
        swapped.insert(first_xor.result.clone());
        //find swapped z
        let z_mess = find_gate(Some(&first_xor.result), None, None, "XOR", gates).unwrap();
        swapped.insert(z_mess.result);
        return;
    }
    let first_and = find_gate(Some(&x), Some(&y), None, "AND", gates).unwrap();
    if first_and.result.starts_with("z") {
        swapped.insert(first_and.result);
        let z_mess = find_gate(Some(&first_xor.result), None, None, "XOR", gates).unwrap();
        swapped.insert(z_mess.result);
        return;
    }
    let or_result = find_gate(Some(&first_and.result), None, None, "OR", gates);
    if or_result
        .as_ref()
        .is_some_and(|g| g.result.starts_with("z"))
    {
        swapped.insert(or_result.unwrap().result);
        let z_mess = find_gate(Some(&first_xor.result), None, None, "XOR", gates).unwrap();
        swapped.insert(z_mess.result);
    }

    let next_and = find_gate(Some(&first_xor.result), None, None, "AND", gates);
    if next_and.as_ref().is_some_and(|g| g.result.starts_with("z")) {
        swapped.insert(next_and.unwrap().result);
        let z_mess = find_gate(Some(&first_xor.result), None, None, "XOR", gates).unwrap();
        swapped.insert(z_mess.result);
    }

    if !swapped.contains(&z) {
        let z_gate = find_gate(Some(&first_xor.result), None, Some(&z), "XOR", gates);
        if z_gate.is_none() {
            panic!("{}", z)
        }
    }

    // let next_xor = find_gate(Some(&first_xor.result), None, Some(&z), "XOR", gates);
    // if next_xor.is_none() {
    //     swapped.insert(first_xor.result);
    // } else {
    //     let next_xor_and = find_gate(Some(&first_xor.result), None, None, "AND", gates).unwrap();
    //     let next_xor_and_or = find_gate(Some(&next_xor_and.result), None, None, "OR", gates);
    //     if next_xor_and_or.is_none() {
    //         swapped.insert(next_xor_and.result);
    //     }
    // }

    // if next_xor.clone().is_some_and(|g| !g.result.starts_with("z")) {
    //     swapped.insert(next_xor.clone().unwrap().result);
    // }
    // if first_xor.is_none() {
    //     let is_y = find_gate(Some(&x), None, Some(&z), "XOR", gates);
    //     if is_y.is_some() {
    //         swapped.insert(y);
    //         let mut swapped_gate = is_y.unwrap().gates.difference(&HashSet::from([x]));
    //         swapped.insert(swapped_gate.next().unwrap().to_string());
    //     }
    //     let is_x = find_gate(Some(&y), None, Some(&z), "XOR", gates);
    // }

    // let next_or = find_gate(Some(&first_and.result), None, None, "OR", gates);
    // if next_or.is_none() || first_and.result.starts_with("z") {
    //     swapped.insert(first_and.result);
    // }
    // if next_or.clone().is_some_and(|g| g.result.starts_with("z")) {
    //     swapped.insert(next_or.clone().unwrap().result);
    // }
    // if next_or.is_none() && next_xor.is_none() {
    //     return;
    // }

    // let carry_val = next_or.clone().unwrap().result;
    // let carry_xor = find_gate(Some(&carry_val), None, None, "XOR", gates);
    // let carry_and = find_gate(Some(&carry_val), None, None, "AND", gates);
    // if carry_and.is_none() || carry_xor.is_none() {
    //     swapped.insert(carry_val);
    // }
}

pub fn find_gate(
    g1: Option<&str>,
    g2: Option<&str>,
    result: Option<&str>,
    op: &str,
    gates: &[Gate],
) -> Option<Gate> {
    let gate: Option<&Gate>;
    if g1.is_some() && g2.is_some() {
        gate = gates.iter().find(|g| {
            g.op == op
                && HashSet::from([(g1.unwrap()).to_string(), (g2.unwrap()).to_string()]) == g.gates
        });
    } else if result.is_none() {
        gate = gates
            .iter()
            .find(|g| g.op == op && g.gates.contains(g1.unwrap()));
    } else if result.is_some() && g1.is_some() {
        gate = gates
            .iter()
            .find(|g| g.op == op && g.result == result.unwrap() && g.gates.contains(g1.unwrap()));
    } else {
        gate = gates
            .iter()
            .find(|g| g.op == op && g.result == result.unwrap());
    }

    if gate.is_some() {
        Some(gate.unwrap().clone())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2024));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
