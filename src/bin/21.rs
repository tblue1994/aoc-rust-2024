use std::collections::HashMap;

advent_of_code::solution!(21);

pub fn part_one(input: &str) -> Option<i128> {
    solve(input, 2)
}

pub fn part_two(input: &str) -> Option<i128> {
    solve(input, 25)
}

pub fn solve(input: &str, layers: usize) -> Option<i128> {
    let mut total_complexity = 0;
    for code in input.lines() {
        let mut current_value = 'A';
        let mut sequence = vec![];
        for c in code.chars() {
            sequence.extend(get_numpad_move(current_value, c));
            current_value = c;
        }

        // let presses = get_keypad_moves_separation(&sequence, layers);
        current_value = 'A';
        let mut hash = HashMap::new();
        let mut presses = 0;
        for c in sequence {
            presses += get_keypad_moves_sep_recursive(current_value, c, layers, &mut hash);
            current_value = c.0;
        }
        println!("{}, {}", code, presses);
        total_complexity += code[0..3].parse::<i128>().unwrap() * presses;
    }
    Some(total_complexity)
}

pub fn get_numpad_move(current: char, dest: char) -> Vec<(char, i128)> {
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
    let vert_first = ['A', '0'].contains(&current) && ['1', '4', '7'].contains(&dest)
        || (current_loc.0 - dest_loc.0 > 0
            && !(['A', '0'].contains(&dest) && ['1', '4', '7'].contains(&current)));
    if vert_first {
        let v: i128 = current_loc.1 - dest_loc.1;
        if v < 0 {
            //go up
            dirs.push(('^', v.abs()))
        } else if v > 0 {
            //go go down
            dirs.push(('v', v.abs()))
        }
    }
    let h: i128 = current_loc.0 - dest_loc.0;
    if h < 0 {
        //go left
        dirs.push(('<', h.abs()))
    } else if h > 0 {
        //go right
        dirs.push(('>', h.abs()))
    }
    if !vert_first {
        let v: i128 = current_loc.1 - dest_loc.1;
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

pub fn get_keypad_moves_sep_recursive(
    current: char,
    next: (char, i128),
    layers: usize,
    hash: &mut HashMap<(char, (char, i128), usize), i128>,
) -> i128 {
    if layers == 0 {
        return next.1;
    }
    if hash.contains_key(&(current, next, layers)) {
        return *hash.get(&(current, next, layers)).unwrap();
    }
    let mut x = get_arrowpad_move(current, next.0);
    x.push(('A', next.1));
    let mut curr = 'A';
    let mut count = 0;
    for i in x {
        count += get_keypad_moves_sep_recursive(curr, i, layers - 1, hash);
        curr = i.0;
    }

    hash.insert((current, next, layers), count);

    count
}

pub fn get_keypad_moves_separation(start_sequence: &[(char, i128)], layers: usize) -> i128 {
    let mut current_sequence = Vec::from(start_sequence);
    let mut hash: HashMap<(char, char), Vec<(char, i128)>> = HashMap::new();
    for i in 0..layers {
        let mut new_sequence: Vec<(char, i128)> = vec![];
        let mut current_value = 'A';
        for s in current_sequence {
            if hash.contains_key(&(current_value, s.0)) {
                new_sequence.extend(hash.get(&(current_value, s.0)).unwrap());
            } else {
                let moves_for_move = get_arrowpad_move(current_value, s.0);
                hash.insert((current_value, s.0), moves_for_move.clone());
                new_sequence.extend(moves_for_move);
            }

            new_sequence.push(('A', s.1));
            current_value = s.0;
        }
        println!("{:?},{:?}", i, new_sequence.len());
        current_sequence = new_sequence;
    }

    current_sequence.iter().fold(0, |acc, x| acc + x.1)
}

pub fn get_arrowpad_move(current: char, dest: char) -> Vec<(char, i128)> {
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
    let vert_first = ['A', '^'].contains(&current) && ['<'].contains(&dest)
        || (current_loc.0 - dest_loc.0 > 0 && !['<'].contains(&current));
    if vert_first {
        let v: i128 = current_loc.1 - dest_loc.1;
        if v < 0 {
            //go up
            dirs.push(('^', v.abs()))
        } else if v > 0 {
            //go go down
            dirs.push(('v', v.abs()))
        }
    }
    let h: i128 = current_loc.0 - dest_loc.0;
    if h < 0 {
        //go left
        dirs.push(('<', h.abs()))
    } else if h > 0 {
        //go right
        dirs.push(('>', h.abs()))
    }
    if !vert_first {
        let v: i128 = current_loc.1 - dest_loc.1;
        if v < 0 {
            //go up
            dirs.push(('^', v.abs()))
        } else if v > 0 {
            //go go down
            dirs.push(('v', v.abs()))
        }
    }
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
        assert_eq!(result, Some(154115708116294));
    }
}
