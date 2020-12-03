fn toboggan(mountain: &str, (y_slope, x_slope): (usize, usize)) -> u32 {
    mountain
        .lines()
        .step_by(y_slope)
        .enumerate()
        .fold(0, |tree_hits, (y, line)| {
            tree_hits
                + line
                    .chars()
                    .nth((y * x_slope) % line.len())
                    .map_or(0, |c| (c == '#') as u32)
        })
}

fn multi_toboggan(mountain: &str, slopes: Vec<(usize, usize)>) -> u32 {
    slopes.into_iter().map(|s| toboggan(mountain, s)).product()
}

fn main() {
    let input = include_str!("../input/input.txt");
    println!("Part 1 => {}", toboggan(&input, (1, 3)));
    println!(
        "Part 2 => {}",
        multi_toboggan(&input, vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)])
    );
}

#[test]
fn part_1_test() {
    let test_slope = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    assert_eq!(toboggan(&test_slope, (1, 3)), 7);
}

#[test]
fn part_2_test() {
    let test_slope = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    let slopes = vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];
    assert_eq!(multi_toboggan(&test_slope, slopes), 336);
}
