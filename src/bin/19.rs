use std::collections::{HashMap, HashSet};

advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<u32> {
    let mut towels = vec![];
    let mut desired_designs = vec![];
    for (i, line) in input.lines().enumerate() {
        if i == 0 {
            towels = line.split(", ").collect()
        }

        if i > 1 {
            desired_designs.push(line);
        }
    }

    let mut possible_count = 0;
    let mut unmakeable = HashSet::new();
    for design in desired_designs {
        let filtered_towels: Vec<&str> = towels
            .clone()
            .into_iter()
            .filter(|t| design.contains(t))
            .collect();
        if is_design_possible(design, &filtered_towels, &mut unmakeable) {
            possible_count += 1;
        }
    }

    Some(possible_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut towels = vec![];
    let mut desired_designs = vec![];
    for (i, line) in input.lines().enumerate() {
        if i == 0 {
            towels = line.split(", ").collect()
        }

        if i > 1 {
            desired_designs.push(line);
        }
    }

    let mut total_arrangements = 0;
    let mut known_keys = HashMap::new();
    for design in desired_designs {
        total_arrangements += get_design_counts(design, &towels, &mut known_keys)
    }

    Some(total_arrangements)
}

pub fn is_design_possible(design: &str, towels: &[&str], unmakeable: &mut HashSet<String>) -> bool {
    if design.is_empty() {
        return true;
    }
    if unmakeable.contains(design) {
        return false;
    }

    for towel in towels {
        if design.starts_with(towel) {
            let filtered_towels: Vec<&str> = towels
                .into_iter()
                .filter(|t| design.contains(*t))
                .map(|s| *s)
                .collect();
            let possible = is_design_possible(&design[towel.len()..], &filtered_towels, unmakeable);
            if possible {
                return true;
            }
        }
    }
    unmakeable.insert(design.to_string());
    false
}

pub fn get_design_counts(
    design: &str,
    towels: &[&str],
    known_keys: &mut HashMap<String, u64>,
) -> u64 {
    if design.is_empty() {
        return 1;
    }
    if known_keys.contains_key(design) {
        return *known_keys.get(design).unwrap();
    }
    let mut count = 0;

    for towel in towels {
        if design.starts_with(towel) {
            count += get_design_counts(&design[towel.len()..], towels, known_keys);
        }
    }
    known_keys.insert(design.to_string(), count);
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
