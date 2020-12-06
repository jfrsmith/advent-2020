use std::collections::HashMap;
use std::collections::HashSet;

fn parse_group(group_lines: &[&str], any_answer: bool) -> usize {
    if any_answer {
        group_lines
            .join("")
            .chars()
            .filter(|c| c.is_alphabetic())
            .collect::<HashSet<char>>()
            .len()
    } else {
        group_lines
            .join("")
            .chars()
            .filter(|c| c.is_alphabetic())
            .fold(HashMap::new(), |mut acc, c| {
                *acc.entry(c).or_insert(0 as usize) += 1 as usize;
                acc
            })
            .values()
            .filter(|v| **v >= group_lines.len())
            .count()
    }
}

fn sum_groups(input: &str, any_answer: bool) -> usize {
    input
        .lines()
        .collect::<Vec<&str>>()
        .split(|l| l.is_empty())
        .map(|group| parse_group(group, any_answer))
        .sum()
}

fn main() {
    println!(
        "Part 1 => {}",
        sum_groups(include_str!("../input/input.txt"), true)
    );

    println!(
        "Part 2 => {}",
        sum_groups(include_str!("../input/input.txt"), false)
    );
}

#[test]
fn part_1_test() {
    let test_input = "abc

a
b
c

ab
ac

a
a
a
a

b";

    assert_eq!(sum_groups(test_input, true), 11);
}

#[test]
fn part_2_test() {
    let test_input = "abc

a
b
c

ab
ac

a
a
a
a

b";

    assert_eq!(sum_groups(test_input, false), 6);
}
