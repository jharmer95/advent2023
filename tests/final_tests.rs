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

#[test]
fn day5_part1() {
    let input = inputs::get_input::<String>("data/day_5.txt").expect("Could not parse path!");
    let result = day_5::part1(&input);

    assert_eq!(result, 309_796_150);
}

// TODO: Optimize so it doesn't have to be skipped
#[test]
#[cfg(not(debug_assertions))]
fn day5_part2() {
    let input = inputs::get_input::<String>("data/day_5.txt").expect("Could not parse path!");
    let result = day_5::part2(&input);

    assert_eq!(result, 50_716_416);
}

#[test]
fn day6_part1() {
    let input = inputs::get_input::<String>("data/day_6.txt").expect("Could not parse path!");
    let result = day_6::part1(&input);

    assert_eq!(result, 625_968);
}

#[test]
fn day6_part2() {
    let input = inputs::get_input::<String>("data/day_6.txt").expect("Could not parse path!");
    let result = day_6::part2(&input);

    assert_eq!(result, 43_663_323);
}

#[test]
fn day7_part1() {
    let input = inputs::get_input::<String>("data/day_7.txt").expect("Could not parse path!");
    let result = day_7::part1(&input);

    assert_eq!(result, 245_794_640);
}

#[test]
fn day7_part2() {
    let input = inputs::get_input::<String>("data/day_7.txt").expect("Could not parse path!");
    let result = day_7::part2(&input);

    assert_eq!(result, 247_899_149);
}

#[test]
fn day8_part1() {
    let input = inputs::get_input::<String>("data/day_8.txt").expect("Could not parse path!");
    let result = day_8::part1(&input);

    assert_eq!(result, 12_361);
}

#[test]
fn day8_part2() {
    let input = inputs::get_input::<String>("data/day_8.txt").expect("Could not parse path!");
    let result = day_8::part2(&input);

    assert_eq!(result, 18_215_611_419_223);
}

#[test]
fn day9_part1() {
    let input = inputs::get_input::<String>("data/day_9.txt").expect("Could not parse path!");
    let result = day_9::part1(&input);

    assert_eq!(result, 1_987_402_313);
}

#[test]
fn day9_part2() {
    let input = inputs::get_input::<String>("data/day_9.txt").expect("Could not parse path!");
    let result = day_9::part2(&input);

    assert_eq!(result, 900);
}
