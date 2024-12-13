use std::collections::HashMap;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u128> {
    solve(input, 25)
}

pub fn part_two(input: &str) -> Option<u128> {
    solve(input, 75)
}

pub fn solve(input: &str, times: u32) -> Option<u128> {
    let stones: Vec<u128> = input
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut count = 0;
    let mut known_rock_spawn: HashMap<(u128, u32), u128> = HashMap::new();

    for stone in stones {
        count += recursive_solve(stone, times, &mut known_rock_spawn)
    }

    // for _ in 0..times {
    //     let mut new_state: Vec<u128> = vec![];
    //     for i in 0..stones.len() {
    //         let stone = stones[i];
    //         if stone == 0 {
    //             new_state.push(1);
    //             continue;
    //         }
    //         let stone_string = stone.to_string();
    //         if stone_string.len() % 2 == 0 {
    //             new_state.push(stone_string[0..stone_string.len() / 2].parse().unwrap());
    //             new_state.push(
    //                 stone_string[stone_string.len() / 2..stone_string.len()]
    //                     .parse()
    //                     .unwrap(),
    //             );
    //             continue;
    //         }
    //         new_state.push(stone * 2024);
    //     }
    //     stones = new_state;
    // }

    Some(count)
}

pub fn recursive_solve(
    current_value: u128,
    times: u32,
    known_rock_spawn: &mut HashMap<(u128, u32), u128>,
) -> u128 {
    if times == 0 {
        return 1;
    }
    let known = known_rock_spawn.get(&(current_value, times));
    if known.is_some() {
        return *known.unwrap();
    }
    let mut count = 0;
    if current_value == 0 {
        count += recursive_solve(1, times - 1, known_rock_spawn)
    } else {
        let stone_string = current_value.to_string();
        if stone_string.len() % 2 == 0 {
            count += recursive_solve(
                stone_string[0..stone_string.len() / 2].parse().unwrap(),
                times - 1,
                known_rock_spawn,
            );
            count += recursive_solve(
                stone_string[stone_string.len() / 2..stone_string.len()]
                    .parse()
                    .unwrap(),
                times - 1,
                known_rock_spawn,
            );
        } else {
            count += recursive_solve(current_value * 2024, times - 1, known_rock_spawn)
        }
    }

    known_rock_spawn.insert((current_value, times), count);
    return count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
