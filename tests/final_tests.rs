use advent2023::*;

#[test]
fn day1_part1() {
    let input = inputs::get_input::<String>("data/day_1.txt").expect("Could not parse path!");
    let result = day_1::part1(&input);

    assert_eq!(result, 56108);
}

#[test]
fn day1_part2() {
    let input = inputs::get_input::<String>("data/day_1.txt").expect("Could not parse path!");
    let result = day_1::part2(&input);

    assert_eq!(result, 55652);
}

#[test]
fn day2_part1() {
    let input = inputs::get_input::<String>("data/day_2.txt").expect("Could not parse path!");
    let result = day_2::part1(&day_2::TEST_LIMIT, &input);

    assert_eq!(result, 2331);
}

#[test]
fn day2_part2() {
    let input = inputs::get_input::<String>("data/day_2.txt").expect("Could not parse path!");
    let result = day_2::part2(&input);

    assert_eq!(result, 71585);
}
