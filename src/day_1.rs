#[must_use]
pub fn part1(input: &[String]) -> u64 {
    let numbers = parse_input1(input);
    numbers.iter().sum()
}

#[must_use]
pub fn part2(input: &[String]) -> u64 {
    let numbers = parse_input2(input);
    numbers.iter().sum()
}

fn parse_input1(input: &[String]) -> Vec<u64> {
    let mut numbers = Vec::with_capacity(input.len());

    for line in input {
        let mut digits = line
            .chars()
            .filter(char::is_ascii_digit)
            .map(|c| c.to_digit(10).unwrap());

        let first = digits.next().unwrap();
        let last = digits.last().unwrap_or(first);

        numbers.push(u64::from(first * 10 + last));
    }

    numbers
}

fn parse_input2(input: &[String]) -> Vec<u64> {
    let mut new_input = Vec::new();

    for line in input {
        new_input.push(
            line.replace("one", "o1e")
                .replace("two", "t2o")
                .replace("three", "t3e")
                .replace("four", "4")
                .replace("five", "5e")
                .replace("six", "6")
                .replace("seven", "7n")
                .replace("eight", "e8t")
                .replace("nine", "n9e"),
        );
    }

    parse_input1(&new_input)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input1() -> [String; 4] {
        [
            "1abc2".to_owned(),
            "pqr3stu8vwx".to_owned(),
            "a1b2c3d4e5f".to_owned(),
            "treb7uchet".to_owned(),
        ]
    }

    fn get_test_input2() -> [String; 7] {
        [
            "two1nine".to_owned(),
            "eightwothree".to_owned(),
            "abcone2threexyz".to_owned(),
            "xtwone3four".to_owned(),
            "4nineeightseven2".to_owned(),
            "zoneight234".to_owned(),
            "7pqrstsixteen".to_owned(),
        ]
    }

    #[test]
    fn parse_input1_test() {
        let result = parse_input1(&get_test_input1());

        assert_eq!(result, vec![12, 38, 15, 77]);
    }

    #[test]
    fn part1_ex_test() {
        let result = part1(&get_test_input1());

        assert_eq!(result, 142);
    }

    #[test]
    fn parse_input2_test() {
        let result = parse_input2(&get_test_input2());

        assert_eq!(result, vec![29, 83, 13, 24, 42, 14, 76]);
    }

    #[test]
    fn part2_ex_test() {
        let result = part2(&get_test_input2());

        assert_eq!(result, 281);
    }
}
