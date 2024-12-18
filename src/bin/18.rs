use std::{
    collections::{HashMap, HashSet},
    usize,
};

advent_of_code::solution!(18);

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, 71, 1024)
}

pub fn test_one(input: &str) -> Option<u32> {
    solve(input, 7, 12)
}

pub fn test_two(input: &str) -> Option<u32> {
    find_trapping_block_line_num(input, 7, 12)
}

pub fn part_two(input: &str) -> Option<u32> {
    find_trapping_block_line_num(input, 71, 1024)
}

pub fn find_trapping_block_line_num(
    input: &str,
    grid_size: usize,
    known_start: usize,
) -> Option<u32> {
    let mut min = known_start;
    let mut max = input.lines().count();
    loop {
        //find middle value
        if max - min == 1 {
            break;
        }
        let current_val = ((max - min) / 2) + min;
        let path = solve(input, grid_size, current_val).unwrap();
        if path == u32::MAX {
            max = current_val
        } else {
            min = current_val
        }
    }
    Some((max) as u32)
}

pub fn solve(input: &str, grid_size: usize, num_blocks: usize) -> Option<u32> {
    let blocks: Vec<(usize, usize)> = input
        .lines()
        .map(|l| l.split(","))
        .map(|mut s| {
            (
                s.next().unwrap().parse::<usize>().unwrap(),
                s.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<(usize, usize)>>();
    let mut bad_blocks = HashSet::new();
    for i in 0..num_blocks {
        bad_blocks.insert(blocks[i]);
    }

    let mut visited = HashMap::new();
    let steps = shortest_path((0, 0), 0, grid_size, &mut visited, &bad_blocks);

    Some(steps)
}

pub fn shortest_path(
    position: (usize, usize),
    current_steps: u32,
    grid_size: usize,
    visited: &mut HashMap<(usize, usize), u32>,
    bad_blocks: &HashSet<(usize, usize)>,
) -> u32 {
    visited.insert(position, current_steps);

    if position.0 == grid_size - 1 && position.1 == grid_size - 1 {
        return current_steps;
    }
    let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    let paths = directions.into_iter().map(|dir| {
        let x = position.0.checked_add_signed(dir.0);
        let y = position.1.checked_add_signed(dir.1);
        if x.is_some_and(|x| x < grid_size)
            && y.is_some_and(|x| x < grid_size)
            && (!visited.contains_key(&(x.unwrap(), y.unwrap()))
                || *visited.get(&(x.unwrap(), y.unwrap())).unwrap() > current_steps + 1)
            && !bad_blocks.contains(&position)
        {
            return shortest_path(
                (x.unwrap(), y.unwrap()),
                current_steps + 1,
                grid_size,
                visited,
                bad_blocks,
            );
        }
        return u32::MAX;
    });
    paths.min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = test_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = test_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }
}
