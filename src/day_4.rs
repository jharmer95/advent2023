#[must_use]
pub fn part1(input: &[String]) -> i64 {
    let mut score = 0;

    for line in input {
        let (win, play) = parse_numbers(line);
        let match_count = get_win_count(&win, &play);

        if match_count != 0 {
            score += 2i64.pow(match_count - 1);
        }
    }

    score
}

#[must_use]
pub fn part2(input: &[String]) -> i64 {
    let length = input.len();
    let mut trackers = vec![1; length];

    for i in 0..length {
        let (win, play) = parse_numbers(&input[i]);
        let win_count = get_win_count(&win, &play);

        for j in 0..win_count as usize {
            trackers[i + j + 1] += trackers[i];
        }
    }

    trackers.iter().sum()
}

fn parse_numbers(line: &str) -> (Vec<i64>, Vec<i64>) {
    let win_start = line.find(':').unwrap();
    let play_start = line.find('|').unwrap();

    (
        get_numbers(&line[win_start..play_start]),
        get_numbers(&line[play_start..]),
    )
}

fn get_numbers(line: &str) -> Vec<i64> {
    let num_split: Vec<&str> = line.split(' ').collect();
    let mut num_list = Vec::new();

    for seq in num_split
        .iter()
        .filter(|s| !s.is_empty() && s.chars().all(|c| c.is_ascii_digit()))
    {
        num_list.push(seq.parse().unwrap());
    }

    num_list
}

#[allow(clippy::cast_possible_truncation)]
fn get_win_count(win_nums: &[i64], play_nums: &[i64]) -> u32 {
    play_nums.iter().filter(|n| win_nums.contains(n)).count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input() -> [String; 6] {
        [
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_owned(),
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19".to_owned(),
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1".to_owned(),
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83".to_owned(),
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36".to_owned(),
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_owned(),
        ]
    }

    #[test]
    fn parse_numbers_test() {
        let expected_wins = vec![41, 48, 83, 86, 17];
        let expected_nums = vec![83, 86, 6, 31, 17, 9, 48, 53];

        let (wins, nums) = parse_numbers(&get_test_input()[0]);

        assert_eq!(wins, expected_wins);
        assert_eq!(nums, expected_nums);
    }

    #[test]
    fn part1_ex_test() {
        let result = part1(&get_test_input());

        assert_eq!(result, 13);
    }

    #[test]
    fn part2_ex_test() {
        let result = part2(&get_test_input());

        assert_eq!(result, 30);
    }
}
