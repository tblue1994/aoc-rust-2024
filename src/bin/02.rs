advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut count = 0;
    let reports = parse(input);
    for mut report in reports {
        let valid = solve(&mut report);
        if valid {
            count += 1
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut count = 0;
    let reports = parse(input);
    for report in reports {
        // if report.first() > report.last() {
        //     report.reverse();
        // }

        // let mut valid = solve(&mut report);
        // if valid {
        //     count += 1;
        //     break;
        // }
        for j in 0..report.len() {
            let mut report_short: Vec<i32> = report
                .clone()
                .into_iter()
                .enumerate()
                .filter(|&(i, _)| i != j)
                .map(|(_, e)| e)
                .collect();
            let valid = solve(&mut report_short);
            if valid {
                count += 1;
                break;
            }
        }
    }
    Some(count)
}

pub fn parse(input: &str) -> Vec<Vec<i32>> {
    let mut reports: Vec<Vec<i32>> = Vec::new();
    let lines = input.lines();
    for line in lines {
        let mut report: Vec<i32> = Vec::new();
        let split = line.split_whitespace();
        split.for_each(|f| report.push(f.parse::<i32>().unwrap()));
        reports.push(report);
    }
    reports
}

pub fn solve(report: &mut Vec<i32>) -> bool {
    if report[0] > report[1] {
        report.reverse();
    }
    for i in 0..report.len() - 1 {
        let dif = report[i + 1] - report[i];
        if dif < 1 || dif > 3 {
            return false;
        }
    }
    return true;
}

pub fn solve_with_mulligan(report: &Vec<i32>) -> bool {
    let mut mulligan = false;
    let mut skip = false;
    for i in 0..report.len() - 1 {
        if skip {
            skip = false;
            continue;
        }
        let dif = report[i + 1] - report[i];
        if dif < 1 || dif > 3 {
            if !mulligan {
                if i == 0 || i == (report.len() - 2) {
                    mulligan = true;
                    continue;
                } else {
                    let skip_dif = report[i + 1] - report[i - 1];
                    if skip_dif >= 1 && skip_dif <= 3 {
                        mulligan = true;
                        continue;
                    } else if !(report.len() <= (i + 2)) {
                        // handle "next" being bad
                        let forward_dif = report[i + 2] - report[i];
                        if forward_dif >= 1 && forward_dif <= 3 {
                            mulligan = true;
                            skip = true;
                            continue;
                        }
                    }
                }
            }
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
