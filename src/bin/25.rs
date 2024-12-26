advent_of_code::solution!(25);

pub fn part_one(input: &str) -> Option<u32> {
    let mut keys: Vec<Vec<usize>> = vec![];
    let mut locks: Vec<Vec<usize>> = vec![];
    let blocks: Vec<&str> = input.split("\n\n").collect();
    for block in blocks {
        let mut vals = vec![0, 0, 0, 0, 0];
        for (i, line) in block.lines().enumerate() {
            if (1..6).contains(&i) {
                for (j, c) in line.chars().enumerate() {
                    if c == '#' {
                        vals[j] += 1;
                    }
                }
            }
        }
        if block.starts_with("#") {
            locks.push(vals);
        } else {
            keys.push(vals);
        }
    }
    let mut count = 0;
    for key in &keys {
        for lock in &locks {
            let mut lock_works = true;
            for i in 0..key.len() {
                if key[i] + lock[i] > 5 {
                    lock_works = false;
                    break;
                }
            }
            if lock_works {
                count += 1;
            }
        }
    }
    Some(count)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
