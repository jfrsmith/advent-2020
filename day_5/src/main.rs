use std::collections::HashSet;

fn decode_pass(boarding_pass: &str) -> u32 {
    let (row, col) = boarding_pass.split_at(boarding_pass.find(|c| c == 'L' || c == 'R').unwrap());

    row.chars()
        .fold((0, 127), |range, c| match c {
            'F' => (range.0, range.0 + ((range.1 - range.0) / 2)),
            'B' => (range.1 - ((range.1 - range.0) / 2), range.1),
            x => panic!("Unexpected character {}", x),
        })
        .0
        * 8
        + col
            .chars()
            .fold((0, 7), |range, c| match c {
                'L' => (range.0, range.0 + ((range.1 - range.0) / 2)),
                'R' => (range.1 - ((range.1 - range.0) / 2), range.1),
                x => panic!("Unexpected character {}", x),
            })
            .0
}

fn find_highest_seat_id(boarding_passes: &str) -> u32 {
    get_full_seats(boarding_passes).into_iter().max().unwrap()
}

fn get_full_seats(boarding_passes: &str) -> Vec<u32> {
    boarding_passes.lines().map(|bp| decode_pass(bp)).collect()
}

fn find_my_seat(boarding_passes: &str) -> u32 {
    let full_seats = get_full_seats(boarding_passes);
    *(0..1024)
        .collect::<HashSet<u32>>()
        .difference(&full_seats.iter().cloned().collect::<HashSet<u32>>())
        .filter(|empty_seat| {
            full_seats.contains(&(*empty_seat + 1)) && full_seats.contains(&(*empty_seat - 1))
        })
        .nth(0)
        .unwrap()
}

fn main() {
    println!(
        "Part 1 => {}",
        find_highest_seat_id(include_str!("../input/input.txt"))
    );

    println!(
        "Part 2 => {}",
        find_my_seat(include_str!("../input/input.txt"))
    );
}

#[test]
fn test_boarding_pass() {
    assert_eq!(decode_pass("FBFBBFFRLR"), 357);
    assert_eq!(decode_pass("BFFFBBFRRR"), 567);
    assert_eq!(decode_pass("FFFBBBFRRR"), 119);
    assert_eq!(decode_pass("BBFFBBFRLL"), 820);
}
