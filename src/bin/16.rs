use std::{
    collections::{HashMap, HashSet},
    isize, u64,
};

advent_of_code::solution!(16);

pub fn part_one(input: &str) -> Option<u64> {
    let (score, _) = solve_wrapper(input);
    Some(score)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, nodes) = solve_wrapper(input);
    Some(nodes.len() as u64)
}

pub fn solve_wrapper(input: &str) -> (u64, HashSet<(isize, isize)>) {
    let mut maze: Vec<Vec<char>> = vec![];
    let mut starting_position = (-1, -1);
    for (i, row) in input.lines().enumerate() {
        if row.contains("S") {
            let j = row.chars().position(|c| c == 'S').unwrap();
            starting_position = (j as isize, i as isize);
        }
        maze.push(row.chars().collect());
    }
    let mut visited = HashMap::new();
    let mut current_path = HashSet::new();

    solve(
        &maze,
        starting_position,
        (1, 0),
        0,
        u64::MAX,
        &mut visited,
        &mut current_path,
    )
}

pub fn solve(
    maze: &Vec<Vec<char>>,
    position: (isize, isize),
    direction: (isize, isize),
    score: u64,
    best_score: u64,
    visited: &mut HashMap<(isize, isize), u64>,
    current_path: &mut HashSet<(isize, isize)>,
) -> (u64, HashSet<(isize, isize)>) {
    let existing_score = visited.get(&position);
    if existing_score.is_none() || existing_score.is_some_and(|x| *x > score) {
        visited.insert(position, score);
    }
    current_path.insert(position);
    let mut paths_to_return = current_path.clone();
    if maze[position.1 as usize][position.0 as usize] == 'E' {
        return (score, paths_to_return);
    }

    let mut score_to_return = u64::MAX;
    if score > best_score {
        return (score_to_return, paths_to_return);
    }
    //go forward
    let forward = (position.0 + direction.0, position.1 + direction.1);
    if is_valid_move(forward, score + 1, maze, visited) {
        let (path_score, best_paths) = solve(
            maze,
            forward,
            direction,
            score + 1,
            score_to_return,
            visited,
            &mut current_path.clone(),
        );

        if path_score < score_to_return {
            score_to_return = path_score;
            paths_to_return = best_paths.clone();
        }
        if path_score == score_to_return {
            paths_to_return.extend(best_paths);
        }
    }

    //try turns
    let turn_dirs = vec![get_left(direction), get_right(direction)];
    for dir in turn_dirs {
        let turned_space = (position.0 + dir.0, position.1 + dir.1);
        if is_valid_move(turned_space, score + 1001, maze, visited) {
            let (path_score, best_paths) = solve(
                maze,
                turned_space,
                dir,
                score + 1001,
                score_to_return,
                visited,
                &mut current_path.clone(),
            );

            if path_score < score_to_return {
                score_to_return = path_score;
                paths_to_return = best_paths.clone();
            }
            if path_score == score_to_return {
                paths_to_return.extend(best_paths);
            }
        }
    }

    (score_to_return, paths_to_return)
}

pub fn is_valid_move(
    space: (isize, isize),
    expected_score: u64,
    maze: &Vec<Vec<char>>,
    visited: &mut HashMap<(isize, isize), u64>,
) -> bool {
    visited
        .get(&space)
        .and_then(|v| Some(*v >= expected_score - 1001))
        .or_else(|| Some(true))
        .unwrap()
        && maze[space.1 as usize][space.0 as usize] != '#'
}

pub fn get_left(direction: (isize, isize)) -> (isize, isize) {
    match direction {
        (1, 0) => (0, -1),
        (0, -1) => (-1, 0),
        (-1, 0) => (0, 1),
        _ => (1, 0),
    }
}

pub fn get_right(direction: (isize, isize)) -> (isize, isize) {
    match direction {
        (1, 0) => (0, 1),
        (0, -1) => (1, 0),
        (-1, 0) => (0, -1),
        _ => (-1, 0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45));
    }
}
