use std::collections::HashSet;

advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u32> {
    let mut warehouse: Vec<Vec<char>> = vec![];
    let mut directions: Vec<char> = vec![];
    let mut robot_index = (-1, -1);
    for (y, line) in input.lines().enumerate() {
        if line.contains("#") {
            warehouse.push(line.chars().collect());
            if line.contains("@") {
                let x = line.chars().position(|c| c == '@').unwrap();
                robot_index = (x as isize, y as isize)
            }
        }
        if line.contains("<") {
            directions.extend(line.chars().collect::<Vec<char>>());
        }
    }

    for direction in directions {
        let vector = find_vector_for_direction(direction);
        robot_index = move_object(robot_index, vector, &mut warehouse);
    }

    let mut gps_coordinate_sum = 0;
    for y in 0..warehouse.len() {
        for x in 0..warehouse[0].len() {
            if warehouse[y][x] == 'O' {
                gps_coordinate_sum += x + (y * 100)
            }
        }
    }

    Some(gps_coordinate_sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut warehouse: Vec<Vec<char>> = vec![];
    let mut directions: Vec<char> = vec![];
    let mut robot_index = (-1, -1);
    for (y, line) in input.lines().enumerate() {
        if line.contains("#") {
            let mut row = vec![];
            for c in line.chars() {
                match c {
                    '#' => row.extend(vec!['#', '#']),
                    'O' => row.extend(vec!['[', ']']),
                    '.' => row.extend(vec!['.', '.']),
                    '@' => row.extend(vec!['@', '.']),
                    _ => panic!("not valid map character"),
                }
            }
            if row.contains(&'@') {
                let x = row.iter().position(|c| *c == '@').unwrap();
                robot_index = (x as isize, y as isize)
            }
            warehouse.push(row);
        }
        if line.contains("<") {
            directions.extend(line.chars().collect::<Vec<char>>());
        }
    }

    for direction in directions {
        let vector = find_vector_for_direction(direction);
        robot_index = match direction {
            '<' | '>' => move_object(robot_index, vector, &mut warehouse),
            '^' | 'v' => {
                let new_pos = move_object_2(&[robot_index], vector, &mut warehouse);

                *new_pos.first().unwrap()
            }
            _ => panic!("not a valid direction"),
        };
    }

    let mut gps_coordinate_sum = 0;
    for y in 0..warehouse.len() {
        for x in 0..warehouse[0].len() {
            if warehouse[y][x] == '[' {
                gps_coordinate_sum += x + (y * 100)
            }
        }
    }

    Some(gps_coordinate_sum as u32)
}

pub fn find_vector_for_direction(direction: char) -> (isize, isize) {
    match direction {
        '^' => (0, -1),
        'v' => (0, 1),
        '<' => (-1, 0),
        _ => (1, 0),
    }
}

pub fn move_object(
    current_pos: (isize, isize),
    vector: (isize, isize),
    warehouse: &mut Vec<Vec<char>>,
) -> (isize, isize) {
    let desired_position = (current_pos.0 + vector.0, current_pos.1 + vector.1);
    match warehouse[desired_position.1 as usize][desired_position.0 as usize] {
        '#' => current_pos,
        'O' => {
            let pos = move_object(desired_position, vector, warehouse);
            if pos == desired_position {
                return current_pos;
            }
            warehouse[desired_position.1 as usize][desired_position.0 as usize] =
                warehouse[current_pos.1 as usize][current_pos.0 as usize];
            warehouse[current_pos.1 as usize][current_pos.0 as usize] = '.';
            desired_position
        }
        '[' | ']' => {
            let rock_end = (desired_position.0 + vector.0, desired_position.1 + vector.1);
            //let rock_end_desired = (rock_end.0 + vector.0, rock_end.1 + vector.1);
            let reck_end_pos = move_object(rock_end, vector, warehouse);
            if reck_end_pos == rock_end {
                return current_pos;
            }
            //move other half of rock
            warehouse[rock_end.1 as usize][rock_end.0 as usize] =
                warehouse[desired_position.1 as usize][desired_position.0 as usize];
            //move current
            warehouse[desired_position.1 as usize][desired_position.0 as usize] =
                warehouse[current_pos.1 as usize][current_pos.0 as usize];
            warehouse[current_pos.1 as usize][current_pos.0 as usize] = '.';
            desired_position
        }
        '.' => {
            //perform swap
            warehouse[desired_position.1 as usize][desired_position.0 as usize] =
                warehouse[current_pos.1 as usize][current_pos.0 as usize];
            warehouse[current_pos.1 as usize][current_pos.0 as usize] = '.';
            return desired_position;
        }
        _ => {
            println!(
                "{}",
                warehouse[desired_position.1 as usize][desired_position.0 as usize]
            );
            panic!("WTF is going on")
        }
    }
}

pub fn move_object_2(
    current_positions: &[(isize, isize)],
    vector: (isize, isize),
    warehouse: &mut Vec<Vec<char>>,
) -> Vec<(isize, isize)> {
    let desired_positions: Vec<(isize, isize)> = current_positions
        .iter()
        .map(|(x, y)| (x + vector.0, y + vector.1))
        .collect();

    let warehouse_spaces: Vec<char> = desired_positions
        .iter()
        .map(|c| warehouse[c.1 as usize][c.0 as usize])
        .collect();

    if warehouse_spaces.contains(&'#') {
        return current_positions.to_vec();
    }
    if warehouse_spaces.iter().all(|c| *c == '.') {
        //swap each space with its desired space
        for pos in current_positions {
            let desired = (pos.0 + vector.0, pos.1 + vector.1);
            warehouse[desired.1 as usize][desired.0 as usize] =
                warehouse[pos.1 as usize][pos.0 as usize];
            warehouse[pos.1 as usize][pos.0 as usize] = '.';
        }
        return desired_positions;
    }
    //handle halfsies
    let mut desired_set = HashSet::new();
    for pos in desired_positions {
        let space = warehouse[pos.1 as usize][pos.0 as usize];
        if space == '[' {
            desired_set.insert(pos);
            desired_set.insert((pos.0 + 1, pos.1));
        }
        if space == ']' {
            desired_set.insert(pos);
            desired_set.insert((pos.0 - 1, pos.1));
        }
    }
    let end_vec = desired_set.into_iter().collect::<Vec<(isize, isize)>>();

    let end_positions = move_object_2(&end_vec, vector, warehouse);

    if end_vec.iter().all(|item| end_positions.contains(item)) {
        return current_positions.to_vec();
    }

    for pos in current_positions {
        //swap with desired spot
        let desired = (pos.0 + vector.0, pos.1 + vector.1);
        warehouse[desired.1 as usize][desired.0 as usize] =
            warehouse[pos.1 as usize][pos.0 as usize];
        warehouse[pos.1 as usize][pos.0 as usize] = '.';
    }

    current_positions
        .iter()
        .map(|(x, y)| (x + vector.0, y + vector.1))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
