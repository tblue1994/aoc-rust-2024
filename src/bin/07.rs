advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    solve_7(input, false)
}

pub fn part_two(input: &str) -> Option<u64> {
    solve_7(input, true)
}

pub fn solve_7(input: &str, allow_or: bool) -> Option<u64> {
    let mut calibration_result = 0;
    for line in input.lines() {
        let mut initial_spilt = line.split(": ");
        let line_calibration_result = initial_spilt.next().unwrap().parse::<u64>().unwrap();
        let values: Vec<u64> = initial_spilt
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect();
        calibration_result += evaluate_line(0, line_calibration_result, &values, allow_or)
    }
    Some(calibration_result)
}

pub fn evaluate_line(
    current_value: u64,
    calibration_result: u64,
    values: &[u64],
    allow_or: bool,
) -> u64 {
    if current_value == 0 {
        return evaluate_line(
            values[0],
            calibration_result,
            &values[1..values.len()],
            allow_or,
        );
    }
    if current_value > calibration_result
        || (values.is_empty() && current_value != calibration_result)
    {
        return 0;
    }
    if values.is_empty() && current_value == calibration_result {
        return calibration_result;
    }
    let add_value = evaluate_line(
        current_value + values[0],
        calibration_result,
        &values[1..values.len()],
        allow_or,
    );
    if add_value > 0 {
        return add_value;
    }
    let mul_value = evaluate_line(
        current_value * values[0],
        calibration_result,
        &values[1..values.len()],
        allow_or,
    );

    if mul_value > 0 {
        return mul_value;
    }

    if allow_or {
        return evaluate_line(
            (current_value.to_string() + &values[0].to_string())
                .parse::<u64>()
                .unwrap(),
            calibration_result,
            &values[1..values.len()],
            allow_or,
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
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
