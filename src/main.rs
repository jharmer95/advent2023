#[allow(clippy::wildcard_imports)]
use advent2023::*;

use std::env;

struct AdventDay {
    mod_name: &'static str,
    nice_name: &'static str,

    part1_fn: fn(&[String]) -> i64,
    part2_fn: fn(&[String]) -> i64,
}

const IMPLEMENTED_DAYS: [AdventDay; 9] = [
    AdventDay {
        mod_name: "day_1",
        nice_name: "Day 1",
        part1_fn: day_1::part1,
        part2_fn: day_1::part2,
    },
    AdventDay {
        mod_name: "day_2",
        nice_name: "Day 2",
        part1_fn: |input| day_2::part1(&day_2::TEST_LIMIT, input),
        part2_fn: day_2::part2,
    },
    AdventDay {
        mod_name: "day_3",
        nice_name: "Day 3",
        part1_fn: day_3::part1,
        part2_fn: day_3::part2,
    },
    AdventDay {
        mod_name: "day_4",
        nice_name: "Day 4",
        part1_fn: day_4::part1,
        part2_fn: day_4::part2,
    },
    AdventDay {
        mod_name: "day_5",
        nice_name: "Day 5",
        part1_fn: day_5::part1,
        part2_fn: day_5::part2,
    },
    AdventDay {
        mod_name: "day_6",
        nice_name: "Day 6",
        part1_fn: day_6::part1,
        part2_fn: day_6::part2,
    },
    AdventDay {
        mod_name: "day_7",
        nice_name: "Day 7",
        part1_fn: day_7::part1,
        part2_fn: day_7::part2,
    },
    AdventDay {
        mod_name: "day_8",
        nice_name: "Day 8",
        part1_fn: day_8::part1,
        part2_fn: day_8::part2,
    },
    AdventDay {
        mod_name: "day_9",
        nice_name: "Day 9",
        part1_fn: day_9::part1,
        part2_fn: day_9::part2,
    },
];

fn help() {
    eprintln!("Usage: advent2023 day_<N>");
    std::process::exit(1);
}

fn main() {
    let mut args = env::args();

    if args.len() > 2 {
        help();
    }

    println!("* * * * * * * * * * * * * *");
    println!("*{:^25}*", "Advent of Code 2023");

    if args.len() == 1 {
        for day in IMPLEMENTED_DAYS {
            exec_day(&day);
        }
    } else {
        let mod_name = args.nth(1).unwrap();

        exec_day(
            IMPLEMENTED_DAYS
                .iter()
                .find(|&day| mod_name == day.mod_name)
                .unwrap_or_else(|| {
                    eprintln!("No matching day!");
                    std::process::exit(1);
                }),
        );
    };
}

fn exec_day(day: &AdventDay) {
    let mod_name = day.mod_name;
    let input =
        inputs::get_input(format!("data/{mod_name}.txt").as_str()).expect("Could not parse path!");

    let result1 = (day.part1_fn)(&input);
    let result2 = (day.part2_fn)(&input);

    println!("* * * * * * * * * * * * * *");
    println!("* {:<24}*", day.nice_name);
    println!("*   Part 1: {result1:<14}*");
    println!("*   Part 2: {result2:<14}*");
    println!("* * * * * * * * * * * * * *");
}
