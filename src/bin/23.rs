use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

advent_of_code::solution!(23);

pub fn part_one(input: &str) -> Option<u32> {
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.lines() {
        let mut s = line.split("-");
        let first = s.next().unwrap();
        let second = s.next().unwrap();
        graph
            .entry(first)
            .and_modify(|x| x.push(second))
            .or_insert(vec![second]);
        graph
            .entry(second)
            .and_modify(|x| x.push(first))
            .or_insert(vec![first]);
    }

    let mut t_three: HashSet<BTreeSet<String>> = HashSet::new();
    for key in graph.keys() {
        if key.starts_with("t") {
            let mut current_path = BTreeSet::new();
            current_path.insert(key.to_string());
            let found_cycles = find_loop_of_len(&key, &key, 2, &mut current_path, &graph);
            for c in found_cycles {
                // println!("{:?}", c);
                t_three.insert(c);
            }
        }
    }
    Some(t_three.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut graph: HashMap<&str, BTreeSet<&str>> = HashMap::new();
    for line in input.lines() {
        let mut s = line.split("-");
        let first = s.next().unwrap();
        let second = s.next().unwrap();
        graph
            .entry(first)
            .and_modify(|x| {
                x.insert(second);
            })
            .or_insert(BTreeSet::from([second]));
        graph
            .entry(second)
            .and_modify(|x| {
                x.insert(first);
            })
            .or_insert(BTreeSet::from([first]));
    }

    let mut largest: BTreeSet<&str> = BTreeSet::new();
    let mut evaluated: HashSet<BTreeSet<&str>> = HashSet::new();
    for key in graph.keys() {
        let mut queue: VecDeque<(BTreeSet<&str>, BTreeSet<&str>)> = VecDeque::new();
        let nodes: &BTreeSet<&str> = graph.get(key).unwrap();
        for adjacent_node in graph.get(key).unwrap() {
            let a_nodes = graph.get(adjacent_node).unwrap();
            let init_group: BTreeSet<&str> = BTreeSet::from([*key, *adjacent_node]);
            let init_inter: BTreeSet<&str> = nodes.intersection(&a_nodes).map(|x| *x).collect();
            if !evaluated.contains(&init_group) {
                queue.push_front((init_group, init_inter));
            }
        }
        while !queue.is_empty() {
            println!("{}", queue.len());
            let (group, babies) = queue.pop_front().unwrap();
            evaluated.insert(group.clone());
            if babies.is_empty() {
                if group.len() > largest.len() {
                    largest = group
                }
                continue;
            }
            if group.len() + babies.len() < largest.len() {
                continue;
            }

            let clone = babies.clone();
            for baby in babies {
                let mut new_group = group.clone();
                new_group.insert(baby);
                let adj_baby = graph.get(baby).unwrap();
                let new_babies = clone.intersection(adj_baby).map(|x| *x).collect();
                if !evaluated.contains(&new_group) {
                    queue.push_front((new_group, new_babies));
                }
            }
        }
    }
    println!("{:?}", largest);
    Some(largest.len() as u32)
}

pub fn find_loop_of_len(
    current: &str,
    end: &str,
    len: usize,
    current_path: &mut BTreeSet<String>,
    graph: &HashMap<&str, Vec<&str>>,
) -> Vec<BTreeSet<String>> {
    current_path.insert(current.to_string());
    let mut all_found = vec![];

    if len == 0 {
        if graph.get(current).unwrap().contains(&end) {
            all_found.push(current_path.clone());
        }
        return all_found;
    }

    for node in graph.get(current).unwrap() {
        all_found.extend(find_loop_of_len(
            node,
            end,
            len - 1,
            &mut current_path.clone(),
            graph,
        ));
    }
    all_found
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
