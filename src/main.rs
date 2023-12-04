use advent2023::*;
use std::{env, fmt::Display};

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
enum AdventDay {
    Day1,
    Day2,
    Day3,
    Day4,
    Day5,
    Day6,
    Day7,
    Day8,
    Day9,
    Day10,
    Day11,
    Day12,
    Day13,
    Day14,
    Day15,
    Day16,
    Day17,
    Day18,
    Day19,
    Day20,
    Day21,
    Day22,
    Day23,
    Day24,
    Day25,
}

impl AdventDay {
    fn mod_name(self) -> String {
        self.to_string().to_ascii_lowercase().replace(' ', "_")
    }

    fn from_mod_name(mod_name: &str) -> Result<Self, String> {
        let day_num = mod_name.replace("day", "").replace('_', "").parse::<u32>();

        match day_num {
            Ok(1) => Ok(Self::Day1),
            Ok(2) => Ok(Self::Day2),
            Ok(x) if x > 0 && x <= 25 => Err(format!("Day {x} not yet implemented")),
            _ => Err(format!("invalid mod_name: {mod_name}")),
        }
    }

    fn exec_part1(self, input: &[String]) -> u64 {
        match self {
            Self::Day1 => day_1::part1(input),
            Self::Day2 => day_2::part1(&day_2::TEST_LIMIT, input),
            _ => unreachable!(),
        }
    }

    fn exec_part2(self, input: &[String]) -> u64 {
        match self {
            Self::Day1 => day_1::part2(input),
            Self::Day2 => day_2::part2(input),
            _ => unreachable!(),
        }
    }
}

impl Display for AdventDay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Day1 => "Day 1",
                Self::Day2 => "Day 2",
                Self::Day3 => "Day 3",
                Self::Day4 => "Day 4",
                Self::Day5 => "Day 5",
                Self::Day6 => "Day 6",
                Self::Day7 => "Day 7",
                Self::Day8 => "Day 8",
                Self::Day9 => "Day 9",
                Self::Day10 => "Day 10",
                Self::Day11 => "Day 11",
                Self::Day12 => "Day 12",
                Self::Day13 => "Day 13",
                Self::Day14 => "Day 14",
                Self::Day15 => "Day 15",
                Self::Day16 => "Day 16",
                Self::Day17 => "Day 17",
                Self::Day18 => "Day 18",
                Self::Day19 => "Day 19",
                Self::Day20 => "Day 20",
                Self::Day21 => "Day 21",
                Self::Day22 => "Day 22",
                Self::Day23 => "Day 23",
                Self::Day24 => "Day 24",
                Self::Day25 => "Day 25",
            }
        )
    }
}

const IMPLEMENTED_DAYS: &[AdventDay] = &[AdventDay::Day1, AdventDay::Day2];

fn help() {
    eprintln!("Usage: advent2023 day_<N>");
    std::process::exit(1);
}

fn main() {
    let mut args = env::args();

    if args.len() > 2 {
        help();
    }

    if args.len() == 1 {
        for &day in IMPLEMENTED_DAYS {
            exec_day(day);
        }
    } else {
        let mod_name = args.nth(1).unwrap();
        exec_day(AdventDay::from_mod_name(&mod_name).unwrap_or_else(|err| {
            eprintln!("{err}");
            std::process::exit(1);
        }));
    };
}

fn exec_day(day: AdventDay) {
    let mod_name = day.mod_name();
    let input =
        inputs::get_input(format!("data/{mod_name}.txt").as_str()).expect("Could not parse path!");

    let result1 = day.exec_part1(&input);
    let result2 = day.exec_part2(&input);

    println!("* * * * * * * * * * * * * *");
    println!("* {:<24}*", day.to_string());
    println!("*   Part 1: {result1:<14}*");
    println!("*   Part 2: {result2:<14}*");
    println!("* * * * * * * * * * * * * *");
}
