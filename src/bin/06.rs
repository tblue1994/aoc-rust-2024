use std::collections::{HashMap, HashSet};

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let lab_grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let guard_position = find_guard(&lab_grid);
    let positions = solve_part1(&lab_grid, guard_position);

    Some(positions.len().try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let lab_grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let starting_guard_position = find_guard(&lab_grid);
    let positions = solve_part1(&lab_grid, starting_guard_position.clone());

    let mut loop_positions: HashSet<(isize, isize)> = HashSet::new();
    for position in positions {
        let mut is_loop = false;
        let mut guard_position = starting_guard_position.clone();
        let mut run_postions: HashMap<(isize, isize), Vec<(isize, isize)>> = HashMap::new();
        let mut direction = (0, -1);
        while guard_position.0 < lab_grid[0].len().try_into().unwrap()
            && guard_position.0 >= 0
            && guard_position.1 < lab_grid.len().try_into().unwrap()
            && guard_position.1 >= 0
        {
            run_postions
                .entry(guard_position)
                .and_modify(|directions| {
                    if directions.contains(&direction) {
                        is_loop = true;
                    } else {
                        directions.push(direction);
                    }
                })
                .or_insert(vec![direction]);
            if is_loop {
                break;
            }
            // let old_directions = run_postions.(guard_position, direction);
            // if run_postions.get(pos).is_some_and(|x| x.contains(&direction)) {
            //     //is loop can can break
            //     is_loop = true;
            //     break;
            // }
            if guard_position.0 + direction.0 < lab_grid[0].len().try_into().unwrap()
                && guard_position.0 + direction.0 >= 0
                && guard_position.1 + direction.1 < lab_grid.len().try_into().unwrap()
                && guard_position.1 + direction.1 >= 0
                && (lab_grid[(guard_position.1 + direction.1) as usize]
                    [(guard_position.0 + direction.0) as usize]
                    == '#'
                    || (guard_position.1 + direction.1 == position.1
                        && guard_position.0 + direction.0 == position.0))
            {
                direction = match direction {
                    (0, -1) => (1, 0),
                    (1, 0) => (0, 1),
                    (0, 1) => (-1, 0),
                    _ => (0, -1),
                };
                continue;
            }
            guard_position = (
                guard_position.0 + direction.0,
                guard_position.1 + direction.1,
            )
        }
        if is_loop {
            loop_positions.insert(position);
        }
    }

    Some(loop_positions.len().try_into().unwrap())
}

pub fn solve_part1(
    lab_grid: &Vec<Vec<char>>,
    mut guard_position: (isize, isize),
) -> HashSet<(isize, isize)> {
    let mut positions: HashSet<(isize, isize)> = HashSet::new();
    let mut direction = (0, -1);
    while guard_position.0 < lab_grid[0].len().try_into().unwrap()
        && guard_position.0 >= 0
        && guard_position.1 < lab_grid.len().try_into().unwrap()
        && guard_position.1 >= 0
    {
        positions.insert(guard_position);

        if guard_position.0 + direction.0 < lab_grid[0].len().try_into().unwrap()
            && guard_position.0 + direction.0 >= 0
            && guard_position.1 + direction.1 < lab_grid.len().try_into().unwrap()
            && guard_position.1 + direction.1 >= 0
            && lab_grid[(guard_position.1 + direction.1) as usize]
                [(guard_position.0 + direction.0) as usize]
                == '#'
        {
            direction = match direction {
                (0, -1) => (1, 0),
                (1, 0) => (0, 1),
                (0, 1) => (-1, 0),
                _ => (0, -1),
            };
            continue;
        }
        guard_position = (
            guard_position.0 + direction.0,
            guard_position.1 + direction.1,
        )
    }
    return positions;
}

pub fn find_guard(lab_grid: &Vec<Vec<char>>) -> (isize, isize) {
    for y in 0..lab_grid.len() {
        for x in 0..lab_grid[0].len() {
            if lab_grid[y][x] == '^' {
                return (x.try_into().unwrap(), y.try_into().unwrap());
            }
        }
    }
    panic!("Guard Not Found")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
