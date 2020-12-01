use std::collections::HashSet;

fn find_summing_pairs(input_set: &HashSet<i32>, target_sum: i32) -> Option<(i32, i32)> {
    for input in input_set {
        let required = target_sum - input;
        if input_set.contains(&required) {
            return Some((*input, required));
        }
    }

    None
}

fn find_summing_triplet(input_set: &HashSet<i32>, target_sum: i32) -> Option<(i32, i32, i32)> {
    for input in input_set {
        let required = target_sum - input;
        let found_triplet = find_summing_pairs(input_set, required)
            .and_then(|pair| return Some((*input, pair.0, pair.1)));
        if found_triplet.is_some() {
            return found_triplet;
        }
    }

    None
}

fn input_to_set(input_str: &str) -> HashSet<i32> {
    input_str
        .lines()
        .map(|x: &str| {
            x.chars()
                .filter(|c| !c.is_whitespace())
                .collect::<String>()
                .parse::<i32>()
                .unwrap()
        })
        .collect::<HashSet<i32>>()
}

fn solve_part_1(input_str: &str, target_sum: i32) -> i32 {
    let pair = find_summing_pairs(&input_to_set(input_str), target_sum).unwrap();
    pair.0 * pair.1
}

fn solve_part_2(input_str: &str, target_sum: i32) -> i32 {
    let triplet = find_summing_triplet(&input_to_set(input_str), target_sum).unwrap();
    triplet.0 * triplet.1 * triplet.2
}

fn main() {
    let input_txt = include_str!("../input/input.txt");
    println!("Part 1 -> {}", solve_part_1(input_txt, 2020));
    println!("Part 2 -> {}", solve_part_2(input_txt, 2020));
}

#[test]
fn part_1() {
    let test_str = "1721
    979
    366
    299
    675
    1456";

    assert_eq!(solve_part_1(test_str, 2020), 514579);
}

#[test]
fn part_2() {
    let test_str = "1721
    979
    366
    299
    675
    1456";

    assert_eq!(solve_part_2(test_str, 2020), 241861950);
}
