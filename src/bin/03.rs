use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mut total = 0;
    for line in input.lines() {
        total += do_multiply(line);
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total = 0;
    let do_splits = input.split("do()");
    for split in do_splits {
        let mut dont_splits = split.split("don't()");
        total += do_multiply(dont_splits.next().unwrap());
    }

    Some(total)
}

pub fn do_multiply(input: &str) -> u32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut total = 0;
    let mut results = vec![];
    for (_, [first, last]) in re.captures_iter(input).map(|c| c.extract()) {
        results.push((first.parse::<u32>().unwrap(), last.parse::<u32>().unwrap()));
    }
    for result in results {
        total += result.0 * result.1;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
