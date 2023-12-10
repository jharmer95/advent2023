use std::cmp::Ordering;
use std::collections::HashMap;

#[must_use]
pub fn part1(input: &[String]) -> i64 {
    let mut hands = parse_input(input, false);
    hands.sort();

    hands
        .iter()
        .enumerate()
        .map(|(x, y)| (x + 1) as i64 * y.wager)
        .sum()
}

#[must_use]
pub fn part2(input: &[String]) -> i64 {
    let mut hands = parse_input(input, true);
    hands.sort();

    hands
        .iter()
        .enumerate()
        .map(|(x, y)| (x + 1) as i64 * y.wager)
        .sum()
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
enum HandValue {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

#[derive(Debug, Default, Eq, PartialEq)]
struct Hand {
    cards: [u8; 5],
    wager: i64,
}

impl Hand {
    fn get_value(&self) -> HandValue {
        let mut char_map: HashMap<u8, u32> = HashMap::new();

        self.cards.iter().for_each(|num| {
            char_map
                .entry(*num)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        });

        let joker_count = *char_map.get(&1).unwrap_or(&0);

        if joker_count == 5 {
            return HandValue::FiveOfKind;
        }

        let mode = char_map
            .iter()
            .filter(|(&k, _)| k != 1)
            .max_by(|(_, v1), (_, v2)| v1.cmp(v2))
            .unwrap();

        char_map
            .entry(*mode.0)
            .and_modify(|count| *count += joker_count);

        let mut last_found = 0;

        for v in char_map
            .iter()
            .filter_map(|(k, v)| if *k == 1 { None } else { Some(v) })
        {
            match v {
                5 => {
                    return HandValue::FiveOfKind;
                }
                4 => {
                    return HandValue::FourOfKind;
                }
                3 => {
                    if last_found == 2 {
                        return HandValue::FullHouse;
                    }

                    last_found = 3;
                }
                2 => {
                    if last_found == 3 {
                        return HandValue::FullHouse;
                    }

                    if last_found == 2 {
                        return HandValue::TwoPair;
                    }

                    last_found = 2;
                }
                _ => {}
            }
        }

        if last_found == 3 {
            return HandValue::ThreeOfKind;
        }

        if last_found == 2 {
            return HandValue::OnePair;
        }

        HandValue::HighCard
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_value = self.get_value();
        let other_value = other.get_value();
        let cmp = self_value.cmp(&other_value);

        if cmp == Ordering::Equal {
            // tie-breaker
            return self.cards.cmp(&other.cards);
        }

        cmp
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_input(input: &[String], part2: bool) -> Vec<Hand> {
    input
        .iter()
        .map(|s| s.split(' ').take(2))
        .map(|mut split| {
            let card_str = split.next().unwrap();
            let wager_str = split.next().unwrap();
            let mut chars = card_str.chars();

            Hand {
                cards: [(); 5].map(|()| match chars.next().unwrap() {
                    'A' => 14,
                    'K' => 13,
                    'Q' => 12,
                    'J' => {
                        if part2 {
                            1
                        } else {
                            11
                        }
                    }
                    'T' => 10,
                    c => u8::try_from(c.to_digit(10).unwrap()).unwrap(),
                }),
                wager: wager_str.parse().unwrap(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inputs;

    fn get_test_input() -> [String; 5] {
        [
            "32T3K 765".to_owned(),
            "T55J5 684".to_owned(),
            "KK677 28".to_owned(),
            "KTJJT 220".to_owned(),
            "QQQJA 483".to_owned(),
        ]
    }

    fn get_test_output() -> Vec<Hand> {
        vec![
            Hand {
                cards: [3, 2, 10, 3, 13],
                wager: 765,
            },
            Hand {
                cards: [10, 5, 5, 11, 5],
                wager: 684,
            },
            Hand {
                cards: [13, 13, 6, 7, 7],
                wager: 28,
            },
            Hand {
                cards: [13, 10, 11, 11, 10],
                wager: 220,
            },
            Hand {
                cards: [12, 12, 12, 11, 14],
                wager: 483,
            },
        ]
    }

    #[test]
    fn parse_input_test() {
        let result = parse_input(&get_test_input(), false);
        let expected_result = get_test_output();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn hand_value_test() {
        let test_hand = Hand {
            cards: [9, 2, 12, 14, 1],
            wager: 1,
        };
        let result = test_hand.get_value();

        assert_eq!(result, HandValue::OnePair);
    }

    #[test]
    fn part1_ex_test() {
        let result = part1(&get_test_input());

        assert_eq!(result, 6_440);
    }

    #[test]
    fn part2_ex_test() {
        let result = part2(&get_test_input());

        assert_eq!(result, 5_905);
    }

    #[test]
    fn part2_xxx_test() {
        let input = inputs::get_input::<String>("data/day_7.txt").expect("Could not parse path!");
        let hands = parse_input(&input, true);

        for hand in hands {
            let _value = hand.get_value();
        }

        assert_eq!(input.len(), 1_000);
    }
}
