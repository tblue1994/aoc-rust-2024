use std::collections::{HashMap, HashSet};

advent_of_code::solution!(20);

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, 2, 100)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input, 20, 100)
}

pub fn test_one(input: &str) -> Option<u32> {
    solve(input, 2, 40)
}

pub fn test_two(input: &str) -> Option<u32> {
    solve(input, 20, 74)
}

pub fn solve(input: &str, max_time: isize, cheat_threshold: u32) -> Option<u32> {
    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);
    let mut map = vec![];
    for (i, line) in input.lines().enumerate() {
        let chars: Vec<char> = line.chars().collect();
        if chars.contains(&'S') {
            start = (chars.iter().position(|c| *c == 'S').unwrap(), i)
        }
        if chars.contains(&'E') {
            end = (chars.iter().position(|c| *c == 'E').unwrap(), i)
        }
        map.push(chars);
    }
    let mut visited = HashSet::new();
    let mut step_counts = HashMap::new();

    shortest_path(start, end, &map, &mut visited, &mut step_counts);

    let mut shortcuts = 0;
    for position in step_counts.clone().into_iter() {
        shortcuts += find_good_shortcuts(position, &step_counts, max_time, cheat_threshold);
    }

    Some(shortcuts)
}

pub fn shortest_path(
    position: (usize, usize),
    end_goal: (usize, usize),
    map: &Vec<Vec<char>>,
    visited: &mut HashSet<(usize, usize)>,
    step_counts: &mut HashMap<(usize, usize), u32>,
) -> u32 {
    visited.insert(position);

    if position == end_goal {
        step_counts.insert(position, 0);
        return 0;
    }
    let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut steps = 0;
    for dir in directions {
        let x = position.0.checked_add_signed(dir.0);
        let y = position.1.checked_add_signed(dir.1);
        if x.is_some_and(|x| x < map[0].len())
            && y.is_some_and(|x| x < map.len())
            && !visited.contains(&(x.unwrap(), y.unwrap()))
            && map[y.unwrap()][x.unwrap()] != '#'
        {
            steps = shortest_path(
                (x.unwrap(), y.unwrap()),
                end_goal,
                map,
                visited,
                step_counts,
            );
        }
    }

    step_counts.insert(position, steps + 1);
    steps + 1
}

pub fn find_good_shortcuts(
    position: ((usize, usize), u32),
    step_counts: &HashMap<(usize, usize), u32>,
    max_time: isize,
    cheat_threshold: u32,
) -> u32 {
    let mut count = 0;

    for dir_x in 0..(max_time + 1) {
        for dir_y in 0..(max_time + 1) {
            if dir_x + dir_y > max_time || (dir_x == 0 && dir_y == 0) {
                continue;
            }
            let mut directions = HashSet::new();
            directions.extend(vec![
                (dir_x, dir_y),
                (dir_x, -dir_y),
                (-dir_x, dir_y),
                (-dir_x, -dir_y),
            ]);
            for dir in directions {
                let x = position.0 .0.checked_add_signed(dir.0);
                let y = position.0 .1.checked_add_signed(dir.1);
                if x.is_some() && y.is_some() && step_counts.contains_key(&(x.unwrap(), y.unwrap()))
                {
                    if position
                        .1
                        .checked_sub(*step_counts.get(&(x.unwrap(), y.unwrap())).unwrap())
                        .is_some_and(|x| x - (dir_x + dir_y) as u32 >= cheat_threshold)
                    {
                        count += 1
                    }
                }
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = test_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = test_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }
}
