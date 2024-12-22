use std::collections::HashMap;

advent_of_code::solution!(21);

pub fn part_one(input: &str) -> Option<u32> {
    let mut total_complexity = 0;
    for code in input.lines() {
        let mut current_value = 'A';
        let mut sequence = vec![];
        for c in code.chars() {
            sequence.extend(get_numpad_move(current_value, c));
            current_value = c;
        }

        let presses = get_keypad_moves_separation(&sequence, 2);
        total_complexity += code[0..3].parse::<u32>().unwrap() * (presses as u32);
    }
    Some(total_complexity)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

pub fn get_numpad_move(current: char, dest: char) -> Vec<(char, i32)> {
    let mut dirs = vec![];
    let numpad_locations = HashMap::from([
        ('A', (0, 0)),
        ('0', (1, 0)),
        ('1', (2, 1)),
        ('2', (1, 1)),
        ('3', (0, 1)),
        ('4', (2, 2)),
        ('5', (1, 2)),
        ('6', (0, 2)),
        ('7', (2, 3)),
        ('8', (1, 3)),
        ('9', (0, 3)),
    ]);
    let current_loc = numpad_locations.get(&current).unwrap();
    let dest_loc = numpad_locations.get(&dest).unwrap();
    let vert_first = vec!['A', '0'].contains(&current) && vec!['1', '4', '7'].contains(&dest)
        || (current_loc.0 - dest_loc.0 > 0
            && !vec!['A', '0'].contains(&dest)
            && !vec!['1', '4', '7'].contains(&current));
    if vert_first {
        let v: i32 = current_loc.1 - dest_loc.1;
        if v < 0 {
            //go up
            dirs.push(('^', v.abs()))
        } else if v > 0 {
            //go go down
            dirs.push(('v', v.abs()))
        }
    }
    let h: i32 = current_loc.0 - dest_loc.0;
    if h < 0 {
        //go left
        dirs.push(('<', h.abs()))
    } else if h > 0 {
        //go right
        dirs.push(('>', h.abs()))
    }
    if !vert_first {
        let v: i32 = current_loc.1 - dest_loc.1;
        if v < 0 {
            //go up
            dirs.push(('^', v.abs()))
        } else if v > 0 {
            //go go down
            dirs.push(('v', v.abs()))
        }
    }
    dirs.push(('A', 1));
    dirs
}

pub fn get_keypad_moves_separation(start_sequence: &[(char, i32)], layers: usize) -> i32 {
    let mut current_sequence = Vec::from(start_sequence);
    for i in 0..layers {
        let mut new_sequence = vec![];
        let mut current_value = 'A';
        for s in current_sequence {
            new_sequence.extend(get_arrowpad_move(current_value, s.0, s.1));
            current_value = s.0;
        }
        println!("{:?}", i);
        current_sequence = new_sequence;
    }

    current_sequence.iter().fold(0, |acc, x| acc + x.1)
}

pub fn get_arrowpad_move(current: char, dest: char, times: i32) -> Vec<(char, i32)> {
    let arrow_locations = HashMap::from([
        ('>', (0, 0)),
        ('v', (1, 0)),
        ('<', (2, 0)),
        ('^', (1, 1)),
        ('A', (0, 1)),
    ]);
    let mut dirs = vec![];
    let current_loc = arrow_locations.get(&current).unwrap();
    let dest_loc = arrow_locations.get(&dest).unwrap();
    let vert_first = vec!['A', '^'].contains(&current) && vec!['<'].contains(&dest)
        || (current_loc.0 - dest_loc.0 > 0 && !vec!['<'].contains(&current));
    if vert_first {
        let v: i32 = current_loc.1 - dest_loc.1;
        if v < 0 {
            //go up
            dirs.push(('^', v.abs()))
        } else if v > 0 {
            //go go down
            dirs.push(('v', v.abs()))
        }
    }
    let h: i32 = current_loc.0 - dest_loc.0;
    if h < 0 {
        //go left
        dirs.push(('<', h.abs()))
    } else if h > 0 {
        //go right
        dirs.push(('>', h.abs()))
    }
    if !vert_first {
        let v: i32 = current_loc.1 - dest_loc.1;
        if v < 0 {
            //go up
            dirs.push(('^', v.abs()))
        } else if v > 0 {
            //go go down
            dirs.push(('v', v.abs()))
        }
    }
    dirs.push(('A', times));
    dirs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
