use std::collections::HashMap;

fn bag_can_contain(
    bag: &str,
    can_contain: &str,
    bag_map: &HashMap<&str, HashMap<&str, usize>>,
) -> bool {
    let bag_contains = bag_map
        .get(bag)
        .expect(format!("Didn't find bag '{}'", bag));

    if bag_contains.get(can_contain).is_some() {
        return true;
    } else {
        return bag_contains
            .iter()
            .any(|(k, _)| bag_can_contain(k, can_contain, bag_map));
    }
}

fn parse_bag_rule(bag_rule: &str) -> (String, HashMap<String, usize>) {
    let split = bag_rule.split(" bags contain ").collect::<Vec<&str>>();
    assert_eq!(split.len(), 2);

    let bag_colour = split[0];
    let bag_contents = split[1].split(", ").collect::<Vec<&str>>();

    if bag_contents.is_empty() {
        return (bag_colour.to_owned(), HashMap::new());
    }

    let bag_contents

    (,)
}

fn part_1(input: &str) -> usize {}

fn main() {
    println!("Part 1 => {}", part_1(include_str!("../input/input.txt")));
}

#[test]
fn part_1_test() {
    assert_eq!(part_1(include_str!("../input/test_input.txt")), 4);
}
