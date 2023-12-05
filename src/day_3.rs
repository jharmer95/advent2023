#[must_use]
pub fn part1(input: &[String]) -> u64 {
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
pub fn part2(input: &[String]) -> u64 {
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
                Some(matches.product::<u64>())
            } else {
                None
            }
        })
        .sum()
}

#[derive(Debug, Eq, PartialEq)]
enum Token {
    Number(u64),
    Symbol(char),
    Spacer,
    End,
}

struct NumberEntry {
    number: u64,
    row: usize,
    range: (usize, usize),
}

impl NumberEntry {
    const fn new(number: u64, row: usize, range: (usize, usize)) -> Self {
        Self { number, row, range }
    }
}

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

fn get_next(str: &[u8], idx: &mut usize) -> Token {
    if *idx < str.len() {
        *idx += 1;
        match str[*idx - 1] {
            b'.' => Token::Spacer,
            ch => {
                if ch.is_ascii_digit() {
                    let mut num_str = String::new();

                    num_str.push(str[*idx - 1] as char);

                    while *idx < str.len() && str[*idx].is_ascii_digit() {
                        num_str.push(str[*idx] as char);
                        *idx += 1;
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

    for (row, line) in input.iter().enumerate() {
        let mut index = 0;
        while tk != Token::End {
            let ind_begin = index;
            tk = get_next(line.as_bytes(), &mut index);
            let ind_end = index - 1;

            match tk {
                Token::Number(n) => {
                    number_list.push(NumberEntry::new(n, row, (ind_begin, ind_end)));
                }

                Token::Symbol(c) => symbol_list.push(SymbolEntry::new(c, row, ind_begin)),
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
            "467..114..".to_string(),
            "...*......".to_string(),
            "..35..633.".to_string(),
            "......#...".to_string(),
            "617*......".to_string(),
            ".....+.58.".to_string(),
            "..592.....".to_string(),
            "......755.".to_string(),
            "...$.*....".to_string(),
            ".664.598..".to_string(),
        ]
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
