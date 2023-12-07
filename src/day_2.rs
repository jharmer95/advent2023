use regex::Regex;

#[must_use]
pub fn part1(limits: &BlockCount, input: &[String]) -> u64 {
    input
        .iter()
        .map(|s| {
            let (id, block_counts) = parse_line(s);

            if block_counts.iter().all(|count| count.within_limit(limits)) {
                id
            } else {
                0
            }
        })
        .sum()
}

#[must_use]
pub fn part2(input: &[String]) -> u64 {
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

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct BlockCount {
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
    const fn within_limit(&self, limit: &Self) -> bool {
        self.red <= limit.red && self.green <= limit.green && self.blue <= limit.blue
    }

    #[must_use]
    const fn power(&self) -> u64 {
        self.red * self.green * self.blue
    }
}

pub const TEST_LIMIT: BlockCount = BlockCount {
    red: 12,
    green: 13,
    blue: 14,
};

fn parse_line(line: &str) -> (u64, Vec<BlockCount>) {
    let re = Regex::new(r"^Game ([0-9]+): (.*)").unwrap();
    let reg_red = Regex::new(r"([0-9]+) red").unwrap();
    let reg_green = Regex::new(r"([0-9]+) green").unwrap();
    let reg_blue = Regex::new(r"([0-9]+) blue").unwrap();

    let mut blocks = Vec::new();
    let mut id = 0;

    let get_color_count = |line, reg_ex: &Regex| match reg_ex.captures(line).map(|c| c.extract()) {
        Some((_, [count])) => count.parse::<u64>().unwrap(),
        None => 0,
    };

    for (_, [idx, rest]) in re.captures_iter(line).map(|caps| caps.extract()) {
        id = idx.parse().unwrap();

        for sequence in rest.split("; ") {
            blocks.push(BlockCount::new(
                get_color_count(sequence, &reg_red),
                get_color_count(sequence, &reg_green),
                get_color_count(sequence, &reg_blue),
            ));
        }
    }

    (id, blocks)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input() -> [String; 5] {
        [
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_owned(),
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_owned(),
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_owned(),
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_owned(),
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_owned(),
        ]
    }

    #[test]
    fn parse_line_test() {
        let result = parse_line(&get_test_input()[0]);

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
        let result = part1(&TEST_LIMIT, &get_test_input());

        assert_eq!(result, 8);
    }

    #[test]
    fn part2_ex_test() {
        let result = part2(&get_test_input());

        assert_eq!(result, 2_286);
    }
}
