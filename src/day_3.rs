#[must_use]
pub fn part1(input: &[String]) -> i64 {
    let (number_list, symbol_list) = parse_input(input);

    number_list
        .iter()
        .filter_map(|num_entry| {
            if symbol_list
                .iter()
                .any(|sym_entry| is_adjacent(sym_entry, num_entry))
            {
                Some(num_entry.number)
            } else {
                None
            }
        })
        .sum()
}

#[must_use]
pub fn part2(input: &[String]) -> i64 {
    let (number_list, symbol_list) = parse_input(input);

    symbol_list
        .iter()
        .filter_map(|sym_entry| {
            if sym_entry.symbol != '*' {
                return None;
            }

            let matches = number_list.iter().filter_map(|num_entry| {
                if is_adjacent(sym_entry, num_entry) {
                    Some(num_entry.number)
                } else {
                    None
                }
            });

            if matches.clone().count() == 2 {
                Some(matches.product::<i64>())
            } else {
                None
            }
        })
        .sum()
}

#[derive(Debug, Eq, PartialEq)]
enum Token {
    Number(i64),
    Symbol(char),
    Spacer,
    End,
}

#[derive(Clone, Debug, PartialEq)]
struct NumberEntry {
    number: i64,
    row: usize,
    range: (usize, usize),
}

impl NumberEntry {
    const fn new(number: i64, row: usize, range: (usize, usize)) -> Self {
        Self { number, row, range }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct SymbolEntry {
    symbol: char,
    row: usize,
    column: usize,
}

impl SymbolEntry {
    const fn new(symbol: char, row: usize, column: usize) -> Self {
        Self {
            symbol,
            row,
            column,
        }
    }
}

fn get_next_token(bytes: &[u8], index: &mut usize) -> Token {
    if *index < bytes.len() {
        *index += 1;
        match bytes[*index - 1] {
            b'.' => Token::Spacer,
            ch => {
                if ch.is_ascii_digit() {
                    let mut num_str = String::new();

                    num_str.push(bytes[*index - 1] as char);

                    while *index < bytes.len() && bytes[*index].is_ascii_digit() {
                        num_str.push(bytes[*index] as char);
                        *index += 1;
                    }

                    Token::Number(num_str.parse().unwrap())
                } else {
                    Token::Symbol(ch as char)
                }
            }
        }
    } else {
        Token::End
    }
}

fn parse_input(input: &[String]) -> (Vec<NumberEntry>, Vec<SymbolEntry>) {
    let mut tk: Token = Token::Spacer;
    let mut number_list = Vec::new();
    let mut symbol_list = Vec::new();

    for (row_num, text) in input.iter().enumerate() {
        let mut index = 0;
        while tk != Token::End {
            let ind_begin = index;
            tk = get_next_token(text.as_bytes(), &mut index);
            let ind_end = index - 1;

            match tk {
                Token::Number(n) => {
                    number_list.push(NumberEntry::new(n, row_num, (ind_begin, ind_end)));
                }

                Token::Symbol(c) => symbol_list.push(SymbolEntry::new(c, row_num, ind_begin)),
                _ => {}
            }
        }

        tk = Token::Spacer;
    }

    (number_list, symbol_list)
}

const fn is_adjacent(sym_entry: &SymbolEntry, num_entry: &NumberEntry) -> bool {
    (sym_entry.row >= num_entry.row.saturating_sub(1)
        && sym_entry.row <= num_entry.row.saturating_add(1))
        && (sym_entry.column >= num_entry.range.0.saturating_sub(1)
            && sym_entry.column <= num_entry.range.1.saturating_add(1))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input() -> [String; 10] {
        [
            "467..114..".to_owned(),
            "...*......".to_owned(),
            "..35..633.".to_owned(),
            "......#...".to_owned(),
            "617*......".to_owned(),
            ".....+.58.".to_owned(),
            "..592.....".to_owned(),
            "......755.".to_owned(),
            "...$.*....".to_owned(),
            ".664.598..".to_owned(),
        ]
    }

    #[test]
    fn parse_input_test() {
        let expected_numbers = vec![
            NumberEntry {
                number: 467,
                row: 0,
                range: (0, 2),
            },
            NumberEntry {
                number: 114,
                row: 0,
                range: (5, 7),
            },
        ];
        let expected_symbols = vec![SymbolEntry {
            symbol: '*',
            row: 1,
            column: 3,
        }];

        let (numbers, symbols) = parse_input(&get_test_input()[0..=1]);

        assert_eq!(numbers, expected_numbers);
        assert_eq!(symbols, expected_symbols);
    }

    #[test]
    fn part1_ex_test() {
        let result = part1(&get_test_input());

        assert_eq!(result, 4_361);
    }

    #[test]
    fn part2_ex_test() {
        let result = part2(&get_test_input());

        assert_eq!(result, 467_835);
    }
}
