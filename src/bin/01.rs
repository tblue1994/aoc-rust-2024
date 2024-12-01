advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let lists = parse(input);
    let mut total = 0;
    for i in 0..lists[0].len() {
        total += lists[0][i].abs_diff(lists[1][i])
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lists = parse(input);
    let mut total = 0;
    for i in 0..lists[0].len() {
        let check_list = lists[1].clone();
        let count = (check_list.into_iter().filter(|&x| x == lists[0][i]).count()) as u32;
        total += lists[0][i] * count;
    }
    Some(total)
}

pub fn parse(input: &str) -> Vec<Vec<u32>> {
    let lines = input.lines();
    let mut list1: Vec<u32> = Vec::new();
    let mut list2: Vec<u32> = Vec::new();
    for line in lines {
        let mut split = line.split_whitespace();
        list1.push(split.next().unwrap().parse::<u32>().unwrap());
        list2.push(split.next().unwrap().parse::<u32>().unwrap());
    }
    list1.sort();
    list2.sort();
    let vec = Vec::from([list1, list2]);
    return vec;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
