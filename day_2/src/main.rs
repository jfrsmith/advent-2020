fn extract_pw_rule(pw_rule: &str) -> (usize, usize, char) {
    let pw_rule: Vec<&str> = pw_rule.split(|c| c == '-' || c == ' ').collect();
    assert_eq!(pw_rule.len(), 3);

    (
        pw_rule[0].parse::<usize>().unwrap(),
        pw_rule[1].parse::<usize>().unwrap(),
        pw_rule[2].parse::<char>().unwrap(),
    )
}

fn validate_part_1(pw_line: &str) -> bool {
    let split_vec: Vec<&str> = pw_line.split(":").collect();
    assert_eq!(split_vec.len(), 2);

    let (min, max, req_char) = extract_pw_rule(split_vec[0]);

    let req_char_count = split_vec[1]
        .trim_start_matches(' ')
        .chars()
        .filter(|c| *c == req_char)
        .count();

    req_char_count >= min && req_char_count <= max
}

fn validate_part_2(pw_line: &str) -> bool {
    let split_vec: Vec<&str> = pw_line.split(":").collect();
    assert_eq!(split_vec.len(), 2);

    let (min, max, req_char) = extract_pw_rule(split_vec[0]);

    let pw_chars = split_vec[1]
        .trim_start_matches(' ')
        .chars()
        .collect::<Vec<char>>();

    (pw_chars[min - 1] == req_char) ^ (pw_chars[max - 1] == req_char)
}

fn part_1_solve(input: &str) -> usize {
    input.lines().filter(|l| validate_part_1(l)).count()
}

fn part_2_solve(input: &str) -> usize {
    input.lines().filter(|l| validate_part_2(l)).count()
}

fn main() {
    let input = include_str!("../input/input.txt");
    println!("Part 1 => {}", part_1_solve(&input));
    println!("Part 2 => {}", part_2_solve(&input));
}

#[test]
fn part_1_test() {
    let input = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

    assert_eq!(part_1_solve(&input), 2);
}

#[test]
fn part_2_test() {
    let input = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

    assert_eq!(part_2_solve(&input), 1);
}
