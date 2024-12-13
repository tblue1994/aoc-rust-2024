use mathru::{
    algebra::linear::{
        matrix::{General, LUDec, Solve},
        vector::Vector,
    },
    matrix, vector,
};
use regex::Regex;
advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<u128> {
    solve(input, false)
}

pub fn part_two(input: &str) -> Option<u128> {
    solve(input, true)
}

pub fn solve(input: &str, oops: bool) -> Option<u128> {
    let mut a_buttons = vec![];
    let mut b_buttons = vec![];
    let mut prizes = vec![];
    for (i, line) in input.lines().enumerate() {
        if i % 4 == 0 {
            //button A
            let re = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").unwrap();
            let (_, [x_str, y_str]) = re.captures_iter(line).map(|c| c.extract()).next().unwrap();
            a_buttons.push((
                x_str.parse::<u128>().unwrap(),
                y_str.parse::<u128>().unwrap(),
            ));
        }
        if i % 4 == 1 {
            //button b
            let re = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").unwrap();
            let (_, [x_str, y_str]) = re.captures_iter(line).map(|c| c.extract()).next().unwrap();
            b_buttons.push((
                x_str.parse::<u128>().unwrap(),
                y_str.parse::<u128>().unwrap(),
            ));
        }
        if i % 4 == 2 {
            //prize
            let re = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
            let (_, [x_str, y_str]) = re.captures_iter(line).map(|c| c.extract()).next().unwrap();
            if oops {
                prizes.push((
                    x_str.parse::<u128>().unwrap() + 10000000000000,
                    y_str.parse::<u128>().unwrap() + 10000000000000,
                ));
            } else {
                prizes.push((
                    x_str.parse::<u128>().unwrap(),
                    y_str.parse::<u128>().unwrap(),
                ));
            }
        }
    }
    let mut cost = 0;
    for i in 0..a_buttons.len() {
        let a: General<f64> = General::new(
            2,
            2,
            vec![
                a_buttons[i].0 as f64,
                a_buttons[i].1 as f64,
                b_buttons[i].0 as f64,
                b_buttons[i].1 as f64,
            ],
        );

        let b: Vector<f64> = vector![prizes[i].0 as f64; prizes[i].1 as f64];
        let x: Vec<f64> = a
            .solve(&b)
            .unwrap()
            .into_iter()
            .map(|x| (x * 100.0).round() / 100.0)
            .collect();
        println!("{:?}", x);
        if x[0].fract() == 0.0 && x[1].fract() == 0.0 {
            cost += (x[0] as u128 * 3) + x[1] as u128;
        }
    }
    Some(cost)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10396088751289990));
    }
}
