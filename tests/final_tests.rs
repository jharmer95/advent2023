use advent2023::*;

#[test]
fn day1_part1() {
    let input = inputs::get_input::<String>("data/day_1.txt").expect("Could not parse path!");
    let result = day_1::part1(&input);

    assert_eq!(result, 56_108);
}

#[test]
fn day1_part2() {
    let input = inputs::get_input::<String>("data/day_1.txt").expect("Could not parse path!");
    let result = day_1::part2(&input);

    assert_eq!(result, 55_652);
}

#[test]
fn day2_part1() {
    let input = inputs::get_input::<String>("data/day_2.txt").expect("Could not parse path!");
    let result = day_2::part1(&day_2::TEST_LIMIT, &input);

    assert_eq!(result, 2_331);
}

#[test]
fn day2_part2() {
    let input = inputs::get_input::<String>("data/day_2.txt").expect("Could not parse path!");
    let result = day_2::part2(&input);

    assert_eq!(result, 71_585);
}

#[test]
fn day3_part1() {
    let input = inputs::get_input::<String>("data/day_3.txt").expect("Could not parse path!");
    let result = day_3::part1(&input);

    assert_eq!(result, 539_637);
}

#[test]
fn day3_part2() {
    let input = inputs::get_input::<String>("data/day_3.txt").expect("Could not parse path!");
    let result = day_3::part2(&input);

    assert_eq!(result, 82_818_007);
}

#[test]
fn day4_part1() {
    let input = inputs::get_input::<String>("data/day_4.txt").expect("Could not parse path!");
    let result = day_4::part1(&input);

    assert_eq!(result, 26_346);
}

#[test]
fn day4_part2() {
    let input = inputs::get_input::<String>("data/day_4.txt").expect("Could not parse path!");
    let result = day_4::part2(&input);

    assert_eq!(result, 8_467_762);
}
