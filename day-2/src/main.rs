use inputs::get_input;
use regex::Regex;

#[derive(Clone, PartialEq, Eq, Debug, Default)]
struct BlockCount {
    red: u64,
    green: u64,
    blue: u64,
}

impl BlockCount {
    #[must_use]
    const fn new(red: u64, green: u64, blue: u64) -> Self {
        Self { red, green, blue }
    }

    #[must_use]
    const fn within(&self, limit: &Self) -> bool {
        self.red <= limit.red && self.green <= limit.green && self.blue <= limit.blue
    }

    #[must_use]
    const fn power(&self) -> u64 {
        self.red * self.green * self.blue
    }
}

const LIMIT_1: BlockCount = BlockCount {
    red: 12,
    green: 13,
    blue: 14,
};

fn main() {
    let input = get_input::<String>("inputs/day-2.txt").expect("Could not parse path!");

    println!("Part 1 solution: {:?}", part1(&LIMIT_1, &input));
    println!("Part 2 solution: {:?}", part2(&input));
}

fn part1(limits: &BlockCount, input: &[String]) -> u64 {
    input
        .iter()
        .map(|s| {
            let (id, block_counts) = parse_line(s);

            if block_counts.iter().all(|count| count.within(limits)) {
                id
            } else {
                0
            }
        })
        .sum()
}

fn part2(input: &[String]) -> u64 {
    input
        .iter()
        .map(|s| {
            let (_, block_counts) = parse_line(s);

            let mut max_block = BlockCount::default();

            for block in block_counts {
                if block.red > max_block.red {
                    max_block.red = block.red;
                }

                if block.green > max_block.green {
                    max_block.green = block.green;
                }

                if block.blue > max_block.blue {
                    max_block.blue = block.blue;
                }
            }

            max_block.power()
        })
        .sum()
}

fn parse_line(line: &str) -> (u64, Vec<BlockCount>) {
    let re = Regex::new(r"^Game ([0-9]+): (.*)").unwrap();
    let reg_red = Regex::new(r"([0-9]+) red").unwrap();
    let reg_green = Regex::new(r"([0-9]+) green").unwrap();
    let reg_blue = Regex::new(r"([0-9]+) blue").unwrap();

    let mut blocks = Vec::new();
    let mut id = 0;

    for (_, [idx, rest]) in re.captures_iter(line).map(|caps| caps.extract()) {
        id = idx.parse().unwrap();

        for xx in rest.split("; ") {
            blocks.push(BlockCount::new(
                match reg_red.captures(xx).map(|c| c.extract()) {
                    Some((_, [count])) => count.parse::<u64>().unwrap(),
                    None => 0,
                },
                match reg_green.captures(xx).map(|c| c.extract()) {
                    Some((_, [count])) => count.parse::<u64>().unwrap(),
                    None => 0,
                },
                match reg_blue.captures(xx).map(|c| c.extract()) {
                    Some((_, [count])) => count.parse::<u64>().unwrap(),
                    None => 0,
                },
            ));
        }
    }

    (id, blocks)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input1() -> [String; 5] {
        [
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string(),
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string(),
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string(),
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_string(),
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string(),
        ]
    }

    #[test]
    fn parse_line_test() {
        let result = parse_line(&get_test_input1()[0]);

        assert_eq!(result.0, 1);
        assert_eq!(result.1.len(), 3);

        assert_eq!(result.1[0].red, 4);
        assert_eq!(result.1[0].green, 0);
        assert_eq!(result.1[0].blue, 3);

        assert_eq!(result.1[1].red, 1);
        assert_eq!(result.1[1].green, 2);
        assert_eq!(result.1[1].blue, 6);

        assert_eq!(result.1[2].red, 0);
        assert_eq!(result.1[2].green, 2);
        assert_eq!(result.1[2].blue, 0);
    }

    #[test]
    fn part1_ex_test() {
        let result = part1(&LIMIT_1, &get_test_input1());

        assert_eq!(result, 8);
    }

    #[test]
    fn part1_final_test() {
        let input = get_input::<String>("../inputs/day-2.txt").expect("Could not parse path!");
        let result = part1(&LIMIT_1, &input);

        assert_eq!(result, 2331);
    }

    #[test]
    fn part2_ex_test() {
        let result = part2(&get_test_input1());

        assert_eq!(result, 2286);
    }

    #[test]
    fn part2_final_test() {
        let input = get_input::<String>("../inputs/day-2.txt").expect("Could not parse path!");
        let result = part2(&input);

        assert_eq!(result, 71585);
    }
}
