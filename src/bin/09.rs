advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i128> {
    let mut memory: Vec<i128> = vec![];
    for (index, val) in input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .enumerate()
    {
        if index % 2 == 0 {
            let mut vec = vec![index as i128 / 2; val as usize];
            memory.append(&mut vec);
        } else {
            let mut vec = vec![-1; val as usize];
            memory.append(&mut vec);
        }
    }
    let mut front = 0;
    let mut rear = memory.len() - 1;
    while front < rear {
        if memory[front] == -1 {
            if memory[rear] != -1 {
                memory.swap(front, rear);
                front += 1;
                rear -= 1;
            } else {
                rear -= 1;
            }
        } else {
            front += 1;
        }
    }
    let mut checksum = 0;
    for (index, val) in memory.into_iter().enumerate() {
        if val != -1 {
            checksum += index as i128 * val;
        }
    }
    Some(checksum)
}

pub fn part_two(input: &str) -> Option<u128> {
    let mut memory: Vec<i128> = vec![];
    for (index, val) in input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .enumerate()
    {
        if index % 2 == 0 {
            let mut vec = vec![index as i128 / 2; val as usize];
            memory.append(&mut vec);
        } else {
            let mut vec = vec![-1; val as usize];
            memory.append(&mut vec);
        }
    }
    // let mut front = 0;
    let mut rear = memory.len() - 1;
    while rear > 0 {
        if memory[rear] != -1 {
            let file_name = memory[rear];
            let mut file_size = 1;
            let mut next = rear - 1;
            while memory[next] == file_name && next > 0 {
                file_size += 1;
                next = next.saturating_sub(1)
            }
            let mut front = 0;
            let mut empty_block_size = 0;
            while empty_block_size < file_size && front <= next {
                if memory[front] == -1 {
                    empty_block_size += 1
                } else {
                    empty_block_size = 0;
                }
                front += 1;
            }
            if empty_block_size == file_size {
                //perform swap
                for i in 0..file_size {
                    memory.swap(front - i - 1, rear - i);
                }
            } else {
                rear = rear.saturating_sub(file_size)
            }
        } else {
            rear = rear.saturating_sub(1)
        }
    }
    let mut checksum = 0;
    for (index, val) in memory.into_iter().enumerate() {
        if val != -1 {
            checksum += index as u128 * val as u128;
        }
    }
    Some(checksum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
