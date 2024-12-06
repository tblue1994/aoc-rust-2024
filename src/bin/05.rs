use std::{cmp::Ordering, collections::HashMap};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, false)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input, true)
}

pub fn solve(input: &str, reorder: bool) -> Option<u32> {
    let mut count = 0;
    let mut rules: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.lines() {
        if line.contains("|") {
            //push new rule
            let mut split = line.split("|");
            let first = split.next().unwrap();
            let second = split.next().unwrap();
            rules
                .entry(first)
                .and_modify(|x| x.push(second))
                .or_insert(vec![second]);
            continue;
        }
        if line.contains(",") {
            let mut pages: Vec<&str> = line.split(",").collect();
            let mut valid = true;
            for i in 1..pages.len() {
                let page_num = pages[i];
                if !rules.contains_key(page_num) {
                    continue;
                }
                for j in 0..i {
                    let previous_page = pages[j];
                    if rules.get(page_num).unwrap().contains(&previous_page) {
                        valid = false;
                        break;
                    }
                }
                if !valid {
                    break;
                }
            }
            if valid && !reorder {
                count += (pages[pages.len() / 2]).parse::<u32>().unwrap();
            } else if !valid && reorder {
                pages.sort_by(|a, b| {
                    if !rules.contains_key(a) {
                        return Ordering::Equal;
                    } else if rules.get(a).unwrap().contains(&b) {
                        return Ordering::Less;
                    } else if rules.get(b).unwrap().contains(&a) {
                        return Ordering::Greater;
                    }
                    Ordering::Equal
                });
                count += (pages[pages.len() / 2]).parse::<u32>().unwrap();
            }
        }
    }
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
