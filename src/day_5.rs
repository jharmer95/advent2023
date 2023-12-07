use std::iter::Skip;
use std::slice::Iter;

#[must_use]
pub fn part1(input: &[String]) -> u64 {
    let almanac = parse_input(input);

    almanac
        .seeds
        .iter()
        .map(|seed| almanac.seed_to_location(*seed))
        .min()
        .unwrap_or(0)
}

#[must_use]
pub fn part2(input: &[String]) -> u64 {
    // TODO: Further optimize
    // Ideas:
    // - Run in parallel
    // - Create a set of ranges instead of full brute force

    let almanac = parse_input(input);

    (0..=u64::MAX)
        .find(|location| almanac.is_seed_in_range(almanac.location_to_seed(*location)))
        .unwrap_or(0)
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct RangeMap {
    dest_start: u64,
    src_start: u64,
    range_size: u64,
}

impl RangeMap {
    const fn new(dest_start: u64, src_start: u64, range_size: u64) -> Self {
        Self {
            dest_start,
            src_start,
            range_size,
        }
    }

    const fn get_destination(&self, source: u64) -> Option<u64> {
        if source < self.src_start || source >= self.src_start + self.range_size {
            None
        } else {
            let offset = source - self.src_start;
            Some(self.dest_start + offset)
        }
    }

    const fn get_source(&self, dest: u64) -> Option<u64> {
        if dest < self.dest_start || dest >= self.dest_start + self.range_size {
            None
        } else {
            let offset = dest - self.dest_start;
            Some(self.src_start + offset)
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
struct Almanac {
    seeds: Vec<u64>,
    seed_to_soil: Vec<RangeMap>,
    soil_to_fertilizer: Vec<RangeMap>,
    fertilizer_to_water: Vec<RangeMap>,
    water_to_light: Vec<RangeMap>,
    light_to_temp: Vec<RangeMap>,
    temp_to_humidity: Vec<RangeMap>,
    humidity_to_location: Vec<RangeMap>,
}

impl Almanac {
    fn seed_to_location(&self, seed: u64) -> u64 {
        let soil = Self::get_mapped_value(seed, &self.seed_to_soil);
        let fertilizer = Self::get_mapped_value(soil, &self.soil_to_fertilizer);
        let water = Self::get_mapped_value(fertilizer, &self.fertilizer_to_water);
        let light = Self::get_mapped_value(water, &self.water_to_light);
        let temp = Self::get_mapped_value(light, &self.light_to_temp);
        let humidity = Self::get_mapped_value(temp, &self.temp_to_humidity);

        Self::get_mapped_value(humidity, &self.humidity_to_location)
    }

    fn location_to_seed(&self, location: u64) -> u64 {
        let humidity = Self::get_rev_mapped_value(location, &self.humidity_to_location);
        let temp = Self::get_rev_mapped_value(humidity, &self.temp_to_humidity);
        let light = Self::get_rev_mapped_value(temp, &self.light_to_temp);
        let water = Self::get_rev_mapped_value(light, &self.water_to_light);
        let fertilizer = Self::get_rev_mapped_value(water, &self.fertilizer_to_water);
        let soil = Self::get_rev_mapped_value(fertilizer, &self.soil_to_fertilizer);

        Self::get_rev_mapped_value(soil, &self.seed_to_soil)
    }

    fn is_seed_in_range(&self, seed: u64) -> bool {
        for chunk in self.seeds.chunks_exact(2) {
            let begin = chunk[0];
            let end = begin + chunk[1];

            if seed >= begin && seed < end {
                return true;
            }
        }

        false
    }

    fn get_mapped_value(key: u64, map: &Vec<RangeMap>) -> u64 {
        for range_map in map {
            let dest = range_map.get_destination(key);

            if let Some(val) = dest {
                return val;
            }
        }

        key
    }

    fn get_rev_mapped_value(key: u64, map: &Vec<RangeMap>) -> u64 {
        for range_map in map {
            let dest = range_map.get_source(key);

            if let Some(val) = dest {
                return val;
            }
        }

        key
    }
}

fn parse_input(input: &[String]) -> Almanac {
    let mut iter = input.iter().skip(3);

    Almanac {
        seeds: input[0]
            .split(' ')
            .skip(1)
            .map(|s| s.parse().unwrap())
            .collect(),
        seed_to_soil: get_next_map(&mut iter),
        soil_to_fertilizer: get_next_map(&mut iter),
        fertilizer_to_water: get_next_map(&mut iter),
        water_to_light: get_next_map(&mut iter),
        light_to_temp: get_next_map(&mut iter),
        temp_to_humidity: get_next_map(&mut iter),
        humidity_to_location: get_next_map(&mut iter),
    }
}

fn get_next_map(iter: &mut Skip<Iter<String>>) -> Vec<RangeMap> {
    let mut map = Vec::new();

    for mut split in iter
        .skip_while(|s| !s.chars().all(|c| c.is_ascii_digit() || c == ' '))
        .take_while(|s| !s.is_empty())
        .map(|s| s.split(' '))
    {
        map.push(RangeMap::new(
            split.next().unwrap().parse().unwrap(),
            split.next().unwrap().parse().unwrap(),
            split.next().unwrap().parse().unwrap(),
        ));
    }

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::manual_string_new)]
    fn get_test_input() -> [String; 33] {
        [
            "seeds: 79 14 55 13".to_owned(),
            "".to_owned(),
            "seed-to-soil map:".to_owned(),
            "50 98 2".to_owned(),
            "52 50 48".to_owned(),
            "".to_owned(),
            "soil-to-fertilizer map:".to_owned(),
            "0 15 37".to_owned(),
            "37 52 2".to_owned(),
            "39 0 15".to_owned(),
            "".to_owned(),
            "fertilizer-to-water map:".to_owned(),
            "49 53 8".to_owned(),
            "0 11 42".to_owned(),
            "42 0 7".to_owned(),
            "57 7 4".to_owned(),
            "".to_owned(),
            "water-to-light map:".to_owned(),
            "88 18 7".to_owned(),
            "18 25 70".to_owned(),
            "".to_owned(),
            "light-to-temperature map:".to_owned(),
            "45 77 23".to_owned(),
            "81 45 19".to_owned(),
            "68 64 13".to_owned(),
            "".to_owned(),
            "temperature-to-humidity map:".to_owned(),
            "0 69 1".to_owned(),
            "1 0 69".to_owned(),
            "".to_owned(),
            "humidity-to-location map:".to_owned(),
            "60 56 37".to_owned(),
            "56 93 4".to_owned(),
        ]
    }

    fn get_test_output() -> Almanac {
        Almanac {
            seeds: vec![79, 14, 55, 13],
            seed_to_soil: vec![RangeMap::new(50, 98, 2), RangeMap::new(52, 50, 48)],
            soil_to_fertilizer: vec![
                RangeMap::new(0, 15, 37),
                RangeMap::new(37, 52, 2),
                RangeMap::new(39, 0, 15),
            ],
            fertilizer_to_water: vec![
                RangeMap::new(49, 53, 8),
                RangeMap::new(0, 11, 42),
                RangeMap::new(42, 0, 7),
                RangeMap::new(57, 7, 4),
            ],
            water_to_light: vec![RangeMap::new(88, 18, 7), RangeMap::new(18, 25, 70)],
            light_to_temp: vec![
                RangeMap::new(45, 77, 23),
                RangeMap::new(81, 45, 19),
                RangeMap::new(68, 64, 13),
            ],
            temp_to_humidity: vec![RangeMap::new(0, 69, 1), RangeMap::new(1, 0, 69)],
            humidity_to_location: vec![RangeMap::new(60, 56, 37), RangeMap::new(56, 93, 4)],
        }
    }

    #[test]
    fn parse_input_test() {
        let expected_result = get_test_output();
        let result = parse_input(&get_test_input());

        assert_eq!(result, expected_result);
    }

    #[test]
    fn part1_ex_test() {
        let result = part1(&get_test_input());

        assert_eq!(result, 35);
    }

    #[test]
    fn part2_ex_test() {
        let result = part2(&get_test_input());

        assert_eq!(result, 46);
    }
}
