use std::collections::HashSet;

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, true)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input, false)
}

pub fn solve(input: &str, use_perimeter: bool) -> Option<u32> {
    let mut total_cost = 0;
    let garden: Vec<Vec<char>> = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    for y in 0..garden.len() {
        for x in 0..garden[0].len() {
            if !visited.contains(&(x, y)) {
                let mut region = HashSet::new();
                find_region(garden[y][x], (x, y), &garden, &mut region);
                let area = region.len() as u32;
                if use_perimeter {
                    let perimeter = find_perimeter(&region, garden[0].len(), garden.len());
                    total_cost += area * perimeter;
                } else {
                    let corners =
                        find_corners(&region, garden[0].len() as isize, garden.len() as isize);
                    total_cost += area * corners;
                }
                visited.extend(region);
            }
        }
    }

    Some(total_cost)
}

pub fn find_region(
    plant: char,
    coordinate: (usize, usize),
    garden: &Vec<Vec<char>>,
    region: &mut HashSet<(usize, usize)>,
) {
    if !region.contains(&coordinate) && garden[coordinate.1][coordinate.0] == plant {
        region.insert(coordinate);
        if coordinate.0 > 0 {
            find_region(plant, (coordinate.0 - 1, coordinate.1), garden, region);
        }
        if coordinate.0 < garden[0].len() - 1 {
            find_region(plant, (coordinate.0 + 1, coordinate.1), garden, region);
        }
        if coordinate.1 > 0 {
            find_region(plant, (coordinate.0, coordinate.1 - 1), garden, region);
        }
        if coordinate.1 < garden.len() - 1 {
            find_region(plant, (coordinate.0, coordinate.1 + 1), garden, region);
        }
    }
}

pub fn find_perimeter(region: &HashSet<(usize, usize)>, max_x: usize, max_y: usize) -> u32 {
    let mut perimeter = 0;
    for coordinate in region {
        let mut p = 4;
        let mut coords_to_check: Vec<(usize, usize)> = vec![];
        if coordinate.0 > 0 {
            coords_to_check.push((coordinate.0 - 1, coordinate.1));
        }
        if coordinate.0 < max_x - 1 {
            coords_to_check.push((coordinate.0 + 1, coordinate.1));
        }
        if coordinate.1 > 0 {
            coords_to_check.push((coordinate.0, coordinate.1 - 1));
        }
        if coordinate.1 < max_y - 1 {
            coords_to_check.push((coordinate.0, coordinate.1 + 1));
        }
        for coord in coords_to_check {
            if region.contains(&coord) {
                p -= 1;
            }
        }
        perimeter += p
    }
    perimeter
}

pub fn find_corners(region: &HashSet<(usize, usize)>, max_x: isize, max_y: isize) -> u32 {
    let mut total_corners = 0;
    for coordinate in region {
        let x = coordinate.0 as isize;
        let y = coordinate.1 as isize;
        total_corners += is_corner(region, max_x, max_y, (x - 1, y), (x, y - 1), (x - 1, y - 1))
            + is_corner(region, max_x, max_y, (x + 1, y), (x, y - 1), (x + 1, y - 1))
            + is_corner(region, max_x, max_y, (x + 1, y), (x, y + 1), (x + 1, y + 1))
            + is_corner(region, max_x, max_y, (x - 1, y), (x, y + 1), (x - 1, y + 1))
    }
    total_corners
}

pub fn is_corner(
    region: &HashSet<(usize, usize)>,
    max_x: isize,
    max_y: isize,
    side1: (isize, isize),
    side2: (isize, isize),
    corner: (isize, isize),
) -> u32 {
    let side_1_edge = side1.0 < 0
        || side1.0 >= max_x
        || side1.1 < 0
        || side1.1 >= max_y
        || !region.contains(&(side1.0 as usize, side1.1 as usize));
    let side_2_edge = side2.0 < 0
        || side2.0 >= max_x
        || side2.1 < 0
        || side2.1 >= max_y
        || !region.contains(&(side2.0 as usize, side2.1 as usize));
    let corner_edge = corner.0 < 0
        || corner.0 >= max_x
        || corner.1 < 0
        || corner.1 >= max_y
        || !region.contains(&(corner.0 as usize, corner.1 as usize));
    if (side_1_edge && side_2_edge) || (!side_1_edge && !side_2_edge && corner_edge) {
        return 1;
    }
    0
}

// pub fn find_sides(
//     region: &HashSet<(usize, usize)>,
//     sides: &mut Vec<((usize, usize), (usize, usize), char)>,
// ) {
//     println!("New Shape");
//     let mut region_vec: Vec<&(usize, usize)> = region.into_iter().collect();
//     region_vec.sort();
//     for coordinate in region_vec {
//         println!("{:?}", coordinate);
//         //left
//         if coordinate.0 == 0 || !region.contains(&(coordinate.0 - 1, coordinate.1)) {
//             update_sides(
//                 sides,
//                 (
//                     (coordinate.0, coordinate.1),
//                     (coordinate.0, coordinate.1 + 1),
//                     'v',
//                 ),
//             )
//             //find existing side
//         }
//         //right
//         if !region.contains(&(coordinate.0 + 1, coordinate.1)) {
//             update_sides(
//                 sides,
//                 (
//                     (coordinate.0 + 1, coordinate.1),
//                     (coordinate.0 + 1, coordinate.1 + 1),
//                     'v',
//                 ),
//             )
//         }

//         //top
//         if coordinate.1 == 0 || !region.contains(&(coordinate.0, coordinate.1 - 1)) {
//             update_sides(
//                 sides,
//                 (
//                     (coordinate.0, coordinate.1),
//                     (coordinate.0 + 1, coordinate.1),
//                     'h',
//                 ),
//             )
//         }

//         //bottom
//         if !region.contains(&(coordinate.0, coordinate.1 + 1)) {
//             update_sides(
//                 sides,
//                 (
//                     (coordinate.0, coordinate.1 + 1),
//                     (coordinate.0 + 1, coordinate.1 + 1),
//                     'h',
//                 ),
//             )
//         }

//         println!("{:?}", sides);
//     }
// }

// pub fn is_connected_side(
//     s: ((usize, usize), (usize, usize), char),
//     comp: ((usize, usize), (usize, usize), char),
// ) -> bool {
//     s.2 == comp.2 && (s.0 == comp.0 || s.0 == comp.1 || s.1 == comp.0 || s.1 == comp.1)
// }

// pub fn update_sides(
//     sides: &mut Vec<((usize, usize), (usize, usize), char)>,
//     side: ((usize, usize), (usize, usize), char),
// ) {
//     let existing_side_pos = sides.into_iter().position(|s| is_connected_side(*s, side));
//     if existing_side_pos.is_none() {
//         sides.push(side)
//     } else {
//         let existing_side = sides[existing_side_pos.unwrap()];
//         let mut all_x = vec![side.0 .0, side.1 .0, existing_side.0 .0, existing_side.1 .0];
//         all_x.sort();
//         let mut all_y = vec![side.0 .1, side.1 .1, existing_side.0 .1, existing_side.1 .1];
//         all_y.sort();
//         sides[existing_side_pos.unwrap()] = (
//             (*all_x.first().unwrap(), *all_y.first().unwrap()),
//             (*all_x.last().unwrap(), *all_y.last().unwrap()),
//             side.2,
//         )
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
