use std::collections::HashSet;

fn num_valid_passports(input_str: &str, skip_field_check: bool) -> usize {
    input_str
        .lines()
        .collect::<Vec<&str>>()
        .split(|l| l.is_empty())
        .map(|passport| {
            passport
                .join(" ")
                .split_whitespace()
                .map(|kvp| {
                    let kvp_vec = kvp.splitn(2, ':').collect::<Vec<&str>>();
                    assert_eq!(kvp_vec.len(), 2);
                    (kvp_vec[0], kvp_vec[1])
                })
                .filter(|(key, value)| {
                    skip_field_check
                        || match *key {
                            "byr" => (1920..=2002).contains(&value.parse::<u32>().unwrap_or(0)),
                            "iyr" => (2010..=2020).contains(&value.parse::<u32>().unwrap_or(0)),
                            "eyr" => (2020..=2030).contains(&value.parse::<u32>().unwrap_or(0)),
                            "hgt" => {
                                if let Some(hgt_met) = value.strip_suffix("cm") {
                                    (150..=193).contains(&hgt_met.parse::<u32>().unwrap_or(0))
                                } else if let Some(hgt_imp) = value.strip_suffix("in") {
                                    (59..=76).contains(&hgt_imp.parse::<u32>().unwrap_or(0))
                                } else {
                                    false
                                }
                            }
                            "hcl" => {
                                if let Some(hex) = value.strip_prefix('#') {
                                    hex.len() == 6
                                        && hex
                                            .chars()
                                            .filter(|c| {
                                                !(c.is_numeric()
                                                    || ['a', 'b', 'c', 'd', 'e', 'f'].contains(c))
                                            })
                                            .count()
                                            == 0
                                } else {
                                    false
                                }
                            }
                            "ecl" => {
                                value.len() == 3
                                    && ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
                                        .contains(value)
                            }
                            "pid" => value.len() == 9 && value.parse::<u32>().is_ok(),
                            "cid" => true,
                            _ => false,
                        }
                })
                .map(|(key, _)| key.to_owned())
                .collect::<HashSet<String>>()
        })
        .filter(|key_set| {
            key_set.is_superset(
                &[
                    "byr".to_owned(),
                    "iyr".to_owned(),
                    "eyr".to_owned(),
                    "hgt".to_owned(),
                    "hcl".to_owned(),
                    "ecl".to_owned(),
                    "pid".to_owned(),
                ]
                .iter()
                .cloned()
                .collect::<HashSet<String>>(),
            )
        })
        .count()
}

fn main() {
    println!(
        "Part 1 => {}",
        num_valid_passports(include_str!("../input/input.txt"), true)
    );

    println!(
        "Part 2 => {}",
        num_valid_passports(include_str!("../input/input.txt"), false)
    );
}

#[test]
fn part_1_test() {
    let input = include_str!("../input/test_input.txt");
    assert_eq!(num_valid_passports(&input, true), 2);
}

#[test]
fn part_2_test() {
    let input = include_str!("../input/test_input_part_2.txt");
    assert_eq!(num_valid_passports(&input, false), 4);
}
