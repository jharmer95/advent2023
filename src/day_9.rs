#[must_use]
pub fn part1(input: &[String]) -> i64 {
    let report = parse_input(input);
    report.iter().map(predict_next).sum()
}

pub fn part2(input: &[String]) -> i64 {
    let report = parse_input(input);
    report.iter().map(predict_prev).sum()
}

fn predict_next(list: &Vec<i64>) -> i64 {
    if list.iter().all(|n| *n == 0) {
        return 0;
    }

    let mut diffs = Vec::with_capacity(list.len() - 1);

    let mut peek = list.iter().peekable();
    let mut current = peek.next().unwrap();

    while let Some(next) = peek.peek() {
        diffs.push(*next - current);
        current = peek.next().unwrap();
    }

    list.last().unwrap() + predict_next(&diffs)
}

fn predict_prev(list: &Vec<i64>) -> i64 {
    if list.iter().all(|n| *n == 0) {
        return 0;
    }

    let mut diffs = Vec::with_capacity(list.len() - 1);

    let mut peek = list.iter().peekable();
    let mut current = peek.next().unwrap();

    while let Some(next) = peek.peek() {
        diffs.push(*next - current);
        current = peek.next().unwrap();
    }

    list.first().unwrap() - predict_prev(&diffs)
}

fn parse_input(input: &[String]) -> Vec<Vec<i64>> {
    input
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input() -> [String; 3] {
        [
            "0 3 6 9 12 15".to_owned(),
            "1 3 6 10 15 21".to_owned(),
            "10 13 16 21 30 45".to_owned(),
        ]
    }

    fn get_test_output() -> Vec<Vec<i64>> {
        vec![
            vec![0, 3, 6, 9, 12, 15],
            vec![1, 3, 6, 10, 15, 21],
            vec![10, 13, 16, 21, 30, 45],
        ]
    }

    #[test]
    fn parse_input_test() {
        let result = parse_input(&get_test_input());
        let expected_result = get_test_output();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn part1_ex_test() {
        let result = part1(&get_test_input());

        assert_eq!(result, 114);
    }

    #[test]
    fn part2_ex_test() {
        let result = part2(&get_test_input());

        assert_eq!(result, 2);
    }
}
