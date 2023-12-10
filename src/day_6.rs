#[must_use]
pub fn part1(input: &[String]) -> i64 {
    let records = parse_input1(input);

    records
        .iter()
        .map(RaceRecord::get_button_time_record_range)
        .map(|(min, max)| max - min + 1)
        .product()
}

#[must_use]
pub fn part2(input: &[String]) -> i64 {
    let record = parse_input2(input);

    let (min, max) = record.get_button_time_record_range();

    max - min + 1
}

fn parse_input1(input: &[String]) -> Vec<RaceRecord> {
    let times = input[0].split_whitespace().filter_map(|s| {
        if s.is_empty() || !s.chars().all(|c| c.is_ascii_digit()) {
            None
        } else {
            s.parse().ok()
        }
    });

    let distances = input[1].split_whitespace().filter_map(|s| {
        if s.is_empty() || !s.chars().all(|c| c.is_ascii_digit()) {
            None
        } else {
            s.parse().ok()
        }
    });

    times
        .zip(distances)
        .map(|(time, distance)| RaceRecord { time, distance })
        .collect()
}

fn parse_input2(input: &[String]) -> RaceRecord {
    let time = input[0]
        .chars()
        .filter(char::is_ascii_digit)
        .collect::<String>()
        .parse()
        .unwrap_or(0);

    let distance = input[1]
        .chars()
        .filter(char::is_ascii_digit)
        .collect::<String>()
        .parse()
        .unwrap_or(0);

    RaceRecord { time, distance }
}

#[derive(Debug, Eq, PartialEq)]
struct RaceRecord {
    time: i64,
    distance: i64,
}

impl RaceRecord {
    #[cfg(not(feature = "fast_math"))]
    fn get_button_time_record_range(&self) -> (i64, i64) {
        let check_record = |time: i64| {
            let distance = (self.time - time) * time;
            distance > self.distance
        };

        // Option 1: Double-ended linear search
        let min = (1..self.time).find(|num| check_record(*num)).unwrap_or(0);

        let max = (1..self.time)
            .rev()
            .find(|num| check_record(*num))
            .unwrap_or(0);

        (min, max)
    }

    #[cfg(feature = "fast_math")]
    fn get_button_time_record_range(&self) -> (i64, i64) {
        // Option 2: Linear equation (distance = time * (self.time - time))
        // - Faster, but potential for off by one errors (ex: f32 fails)
        let ftime = self.time as f64;

        let dev = f64::sqrt(4.0f64.mul_add(-(self.distance as f64), ftime.powi(2)));

        let min = (0.5f64 * (ftime - dev)) as i64 + 1;
        let max = f64::ceil(0.5f64 * (ftime + dev)) as i64 - 1;

        (min, max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input() -> [String; 2] {
        [
            "Time:      7  15   30".to_owned(),
            "Distance:  9  40  200".to_owned(),
        ]
    }

    const fn get_test_output1() -> [RaceRecord; 3] {
        [
            RaceRecord {
                time: 7,
                distance: 9,
            },
            RaceRecord {
                time: 15,
                distance: 40,
            },
            RaceRecord {
                time: 30,
                distance: 200,
            },
        ]
    }

    const fn get_test_output2() -> RaceRecord {
        RaceRecord {
            time: 71_530,
            distance: 940_200,
        }
    }

    #[test]
    fn parse_input1_test() {
        let expected_result = get_test_output1();
        let result = parse_input1(&get_test_input());

        assert_eq!(result, expected_result);
    }

    #[test]
    fn parse_input2_test() {
        let expected_result = get_test_output2();
        let result = parse_input2(&get_test_input());

        assert_eq!(result, expected_result);
    }

    #[test]
    fn part1_ex_test() {
        let result = part1(&get_test_input());

        assert_eq!(result, 288);
    }

    #[test]
    fn part2_ex_test() {
        let result = part2(&get_test_input());

        assert_eq!(result, 71_503);
    }
}
