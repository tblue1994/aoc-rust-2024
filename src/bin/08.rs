use std::{
    collections::{HashMap, HashSet},
    i32, isize,
};

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, 1, false)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input, i32::MAX, true)
}

pub fn solve(input: &str, antinode_depth: i32, include_antennaes: bool) -> Option<u32> {
    let mut antenna_locations: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let max_values = (
        input.lines().next().unwrap().len() as isize,
        input.lines().count() as isize,
    );
    for (y, line) in input.lines().enumerate() {
        for (x, point) in line.chars().enumerate() {
            if point != '.' {
                antenna_locations
                    .entry(point)
                    .and_modify(|points| points.push((x, y)))
                    .or_insert(vec![(x, y)]);
            }
        }
    }
    let mut antinode_locations: HashSet<(isize, isize)> = HashSet::new();

    for (_, points) in antenna_locations {
        for i in 0..points.len() {
            let point_a = points[i];
            for j in i + 1..points.len() {
                let point_b = points[j];
                let diffs = (
                    point_a.0 as isize - point_b.0 as isize,
                    point_a.1 as isize - point_b.1 as isize,
                );
                if include_antennaes {
                    antinode_locations.insert((point_a.0 as isize, point_a.1 as isize));
                    antinode_locations.insert((point_b.0 as isize, point_b.1 as isize));
                }

                // find all points beyond a
                get_antinode_points(
                    &mut antinode_locations,
                    (point_a.0 as isize, point_a.1 as isize),
                    antinode_depth,
                    diffs,
                    max_values,
                );

                // find all points beyond b
                get_antinode_points(
                    &mut antinode_locations,
                    (point_b.0 as isize, point_b.1 as isize),
                    antinode_depth,
                    (-diffs.0, -diffs.1),
                    max_values,
                );
            }
        }
    }
    Some(antinode_locations.len() as u32)
}

pub fn get_antinode_points(
    antinode_locations: &mut HashSet<(isize, isize)>,
    mut current_point: (isize, isize),
    antinode_depth: i32,
    diffs: (isize, isize),
    max_values: (isize, isize),
) {
    for _ in 0..antinode_depth {
        current_point = (current_point.0 + diffs.0, current_point.1 + diffs.1);
        if current_point.0 >= 0
            && current_point.0 < max_values.0
            && current_point.1 >= 0
            && current_point.1 < max_values.1
        {
            antinode_locations.insert(current_point);
        } else {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
