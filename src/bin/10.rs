use std::collections::HashSet;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let topographic_map: Vec<Vec<i32>> = input
        .lines()
        .map(|x| x.chars().map(|c| c.to_digit(10).unwrap() as i32).collect())
        .collect();

    let mut count = 0;
    for y in 0..topographic_map.len() {
        for x in 0..topographic_map[0].len() {
            if topographic_map[y][x] == 0 {
                let mut trailheads_reachable: HashSet<(isize, isize)> = HashSet::new();
                find_trailheads(
                    -1,
                    x as isize,
                    y as isize,
                    &topographic_map,
                    &mut trailheads_reachable,
                );
                count += trailheads_reachable.len()
            }
        }
    }
    Some(count as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let topographic_map: Vec<Vec<i32>> = input
        .lines()
        .map(|x| x.chars().map(|c| c.to_digit(10).unwrap() as i32).collect())
        .collect();

    let mut count = 0;
    for y in 0..topographic_map.len() {
        for x in 0..topographic_map[0].len() {
            if topographic_map[y][x] == 0 {
                count += find_paths(-1, x as isize, y as isize, &topographic_map);
            }
        }
    }
    Some(count)
}

pub fn find_trailheads(
    previous_elevation: i32,
    x: isize,
    y: isize,
    topographic_map: &Vec<Vec<i32>>,
    visited_trailheads: &mut HashSet<(isize, isize)>,
) {
    if x < 0
        || y < 0
        || x >= topographic_map[0].len() as isize
        || y >= topographic_map.len() as isize
    {
        return;
    }
    let new_elevation = topographic_map[y as usize][x as usize];
    if new_elevation != previous_elevation + 1 {
        return;
    }
    if new_elevation == 9 {
        visited_trailheads.insert((x, y));
    }
    find_trailheads(new_elevation, x - 1, y, topographic_map, visited_trailheads);
    find_trailheads(new_elevation, x + 1, y, topographic_map, visited_trailheads);
    find_trailheads(new_elevation, x, y - 1, topographic_map, visited_trailheads);
    find_trailheads(new_elevation, x, y + 1, topographic_map, visited_trailheads);
}

pub fn find_paths(
    previous_elevation: i32,
    x: isize,
    y: isize,
    topographic_map: &Vec<Vec<i32>>,
) -> u32 {
    if x < 0
        || y < 0
        || x >= topographic_map[0].len() as isize
        || y >= topographic_map.len() as isize
    {
        return 0;
    }
    let new_elevation = topographic_map[y as usize][x as usize];
    if new_elevation != previous_elevation + 1 {
        return 0;
    }
    if new_elevation == 9 {
        return 1;
    }
    find_paths(new_elevation, x - 1, y, topographic_map)
        + find_paths(new_elevation, x + 1, y, topographic_map)
        + find_paths(new_elevation, x, y - 1, topographic_map)
        + find_paths(new_elevation, x, y + 1, topographic_map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
