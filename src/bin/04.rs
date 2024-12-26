advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let crossword: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut count = 0;
    let search_term = &['X', 'M', 'A', 'S'];
    let reverse_search_term = &['S', 'A', 'M', 'X'];
    //horizontally
    for y in 0..crossword.len() {
        for x in 0..crossword[0].len() {
            //horizontally
            count += contains_search(x as isize, y as isize, 1, 0, &crossword, search_term);
            count += contains_search(
                x as isize,
                y as isize,
                1,
                0,
                &crossword,
                reverse_search_term,
            );
            //vertically
            count += contains_search(x as isize, y as isize, 0, 1, &crossword, search_term);
            count += contains_search(
                x as isize,
                y as isize,
                0,
                1,
                &crossword,
                reverse_search_term,
            );
            //diagonally
            count += contains_search(x as isize, y as isize, 1, 1, &crossword, search_term);
            count += contains_search(
                x as isize,
                y as isize,
                1,
                1,
                &crossword,
                reverse_search_term,
            );
            //other diagonally
            count += contains_search(x as isize, y as isize, 1, -1, &crossword, search_term);
            count += contains_search(
                x as isize,
                y as isize,
                1,
                -1,
                &crossword,
                reverse_search_term,
            );
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let crossword: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut count = 0;
    for y in 1..crossword.len() - 1 {
        for x in 1..crossword[0].len() - 1 {
            if crossword[y][x] == 'A'
                && is_mas((x + 1, y + 1), (x - 1, y - 1), &crossword)
                && is_mas((x - 1, y + 1), (x + 1, y - 1), &crossword)
            {
                count += 1;
            }
        }
    }

    Some(count)
}

pub fn is_mas(a: (usize, usize), b: (usize, usize), crossword: &[Vec<char>]) -> bool {
    (crossword[a.1][a.0] == 'M' && crossword[b.1][b.0] == 'S')
        || (crossword[a.1][a.0] == 'S' && crossword[b.1][b.0] == 'M')
}

pub fn contains_search(
    x_index: isize,
    y_index: isize,
    x_mod: isize,
    y_mod: isize,
    crossword: &Vec<Vec<char>>,
    search_term: &[char],
) -> u32 {
    if search_term.is_empty() {
        return 1;
    }
    if x_index >= crossword.len() as isize
        || y_index >= crossword[0].len() as isize
        || x_index < 0
        || y_index < 0
    {
        return 0;
    }
    if crossword[y_index as usize][x_index as usize] == *search_term.first().unwrap() {
        return contains_search(
            x_index + x_mod,
            y_index + y_mod,
            x_mod,
            y_mod,
            crossword,
            &search_term[1..],
        );
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
