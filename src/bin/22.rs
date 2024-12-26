use std::collections::{HashMap, HashSet};

advent_of_code::solution!(22);

pub fn part_one(input: &str) -> Option<u128> {
    let mut secret_sum = 0;
    for line in input.lines() {
        let mut value = line.parse().unwrap();
        for _ in 0..2000 {
            let step_1 = prune(mix(value * 64, value));
            let step_2 = prune(mix(step_1 / 32, step_1));
            let step_3 = prune(mix(step_2 * 2048, step_2));
            value = step_3;
        }
        secret_sum += value
    }
    Some(secret_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut best_hash = HashMap::new();
    for line in input.lines() {
        let mut value = line.parse().unwrap();
        let mut ones_digits = vec![get_ones_digit(value)];
        let mut diffs = vec![];
        let mut seen = HashSet::new();
        for _ in 0..2000 {
            let step_1 = prune(mix(value * 64, value));
            let step_2 = prune(mix(step_1 / 32, step_1));
            let step_3 = prune(mix(step_2 * 2048, step_2));
            value = step_3;
            ones_digits.push(get_ones_digit(value));
            diffs.push(
                ones_digits[ones_digits.len() - 2] as i32
                    - ones_digits[ones_digits.len() - 1] as i32,
            )
        }
        for j in 0..diffs.len() - 3 {
            let sequence = Vec::from(diffs.get(j..(j + 4)).unwrap());
            if seen.contains(&sequence) {
                continue;
            }
            seen.insert(sequence.clone());

            let sequence_bananas = *ones_digits.get(j + 4)?;
            best_hash
                .entry(sequence)
                .and_modify(|x| *x += sequence_bananas)
                .or_insert(sequence_bananas);
        }
    }

    Some(*best_hash.values().max()?)

    // let mut best_bananas = 0;
    // for i in 0..best_hashes.len() {
    //     let check = best_hashes.get(i..).unwrap();

    //     for key in best_hashes[i].keys() {
    //         let mut bananas = 0;
    //         for hash in check {
    //             if hash.get(key).is_some() {
    //                 bananas += hash.get(key).unwrap()
    //             }
    //         }
    //         best_bananas = best_bananas.max(bananas)
    //     }
    //     println!("hash check {}", i)
    // }

    // Some(best_bananas)

    // let mut best_banana_hash = HashMap::new();
    // for i in 0..diff_agg.len() - 1 {
    //     for j in 0..diff_agg[i].len() - 3 {
    //         let sequence = &diff_agg[i][j..(j + 4)];
    //         if best_banana_hash.contains_key(sequence) {
    //             continue;
    //         }
    //         let mut sequence_bananas = 0;
    //         for x in i..diff_agg.len() {
    //             sequence_bananas +=
    //                 get_best_banana_count_for_sequence(sequence, &diff_agg[x], &ones_agg[x]);
    //         }
    //         best_banana_hash.insert(sequence, sequence_bananas);
    //     }
    //     println!("{}", i)
    // }
    //
}

pub fn mix(a: u128, b: u128) -> u128 {
    a ^ b
}

pub fn prune(a: u128) -> u128 {
    a % 16777216
}

pub fn get_ones_digit(a: u128) -> u32 {
    a.to_string()
        .chars()
        .last()
        .unwrap()
        .to_string()
        .parse()
        .unwrap()
}

pub fn get_best_banana_count_for_sequence(sequence: &[i32], diffs: &[i32], ones: &[u32]) -> u32 {
    let mut max = 0;
    if sequence == &vec![-2, 1, -1, 3] {
        println!("here")
    }
    for i in 0..diffs.len() - 3 {
        let check = &diffs[i..i + 4];
        if check == sequence {
            max = max.max(ones[i + 4])
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37990510));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(23));
    }
}
