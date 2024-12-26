use std::collections::HashMap;

use regex::Regex;

advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<u32> {
    solve_1(input, 101, 103)
}

pub fn test_one(input: &str) -> Option<u32> {
    solve_1(input, 11, 7)
}

pub fn test_two(_input: &str) -> Option<u32> {
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let guards = parse(input);
    let mut christmas_time = 0;
    for i in 1..i32::MAX {
        let mut positions_at_time: HashMap<i32, Vec<i32>> = HashMap::new();
        for guard in &guards {
            let (pos, vector) = guard;
            let new_x = (pos.0 + (vector.0 * i)).rem_euclid(101);
            let new_y = (pos.1 + (vector.1 * i)).rem_euclid(103);
            positions_at_time
                .entry(new_x)
                .and_modify(|l| l.push(new_y))
                .or_insert(vec![new_y]);
        }
        let valid_rows = positions_at_time
            .values()
            .filter(|v| v.len() > 30 && is_contiguous(v))
            .count();
        println!("{}", valid_rows);
        if valid_rows >= 2 {
            christmas_time = i;
            break;
        }
    }
    Some(christmas_time as u32)
}
pub fn is_contiguous(ints: &[i32]) -> bool {
    let mut row_len = 0;
    for i in 0..103 {
        if ints.contains(&i) {
            row_len += 1;
        } else {
            row_len = 0;
        }
        if row_len > 30 {
            return true;
        }
    }
    false
}

pub fn solve_1(input: &str, x: i32, y: i32) -> Option<u32> {
    let guards = parse(input);
    let mut quadrant_guards = (0, 0, 0, 0);
    for (pos, vector) in guards {
        let end_pos = (
            (pos.0 + (vector.0 * 100)).rem_euclid(x),
            (pos.1 + (vector.1 * 100)).rem_euclid(y),
        );
        //quadrant 1
        if end_pos.0 < x / 2 && end_pos.1 < y / 2 {
            quadrant_guards.0 += 1;
        }
        //quadrant 2
        if end_pos.0 > x / 2 && end_pos.1 < y / 2 {
            quadrant_guards.1 += 1;
        }
        //quadrant 3
        if end_pos.0 < x / 2 && end_pos.1 > y / 2 {
            quadrant_guards.2 += 1;
        }
        //quadrant 4
        if end_pos.0 > x / 2 && end_pos.1 > y / 2 {
            quadrant_guards.3 += 1;
        }
    }

    Some(quadrant_guards.0 * quadrant_guards.1 * quadrant_guards.2 * quadrant_guards.3)
}

pub fn parse(input: &str) -> Vec<((i32, i32), (i32, i32))> {
    let mut guards: Vec<((i32, i32), (i32, i32))> = vec![];
    for line in input.lines() {
        let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
        for (_, [p_x, p_y, v_x, v_y]) in re.captures_iter(line).map(|c| c.extract()) {
            guards.push((
                (p_x.parse::<i32>().unwrap(), p_y.parse::<i32>().unwrap()),
                (v_x.parse::<i32>().unwrap(), v_y.parse::<i32>().unwrap()),
            ));
        }
    }
    guards
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = test_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = test_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
