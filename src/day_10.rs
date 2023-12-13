use std::{cell::RefCell, collections::HashSet};

#[must_use]
pub fn part1(input: &[String]) -> i64 {
    let maze = parse_input(input);
    let mut visited: HashSet<Point2D> = HashSet::new();

    visited.insert(maze.start);

    let [start1, start2] = maze.get_from_start();

    let mut cursor1 = Cursor {
        point: maze.start.get_next(start1),
        direction: start1,
    };

    let mut cursor2 = Cursor {
        point: maze.start.get_next(start2),
        direction: start2,
    };

    let mut step_count = 0;

    while !visited.contains(&cursor1.point) || !visited.contains(&cursor2.point) {
        visited.insert(cursor1.point);
        visited.insert(cursor2.point);

        cursor1 = maze.get_next(&cursor1);
        cursor2 = maze.get_next(&cursor2);
        step_count += 1;
    }

    step_count
}

#[must_use]
pub fn part2(input: &[String]) -> i64 {
    let mut maze = parse_input(input);
    let mut visited: HashSet<Point2D> = HashSet::new();

    visited.insert(maze.start);

    let [start, _] = maze.get_from_start();

    let mut cursor = Cursor {
        point: maze.start.get_next(start),
        direction: start,
    };

    while cursor.point != maze.start {
        visited.insert(cursor.point);

        cursor = maze.get_next(&cursor);
    }

    let maze_y = maze.tiles.len();
    let maze_x = maze.tiles[0].len();

    // TODO: Inside loop function is not correct
    let outside_loop = |point: Point2D| {
        let north_bound = || {
            for y in 0..=point.y {
                let new_point = Point2D { x: point.x, y };

                if visited.contains(&new_point) {
                    return false;
                }
            }

            true
        };

        let south_bound = || {
            for y in point.y..maze_y {
                let new_point = Point2D { x: point.x, y };

                if visited.contains(&new_point) {
                    return false;
                }
            }

            true
        };

        let east_bound = || {
            for x in 0..=point.x {
                let new_point = Point2D { x, y: point.y };

                if visited.contains(&new_point) {
                    return false;
                }
            }

            true
        };

        let west_bound = || {
            for x in point.x..maze_x {
                let new_point = Point2D { x, y: point.y };

                if visited.contains(&new_point) {
                    return false;
                }
            }

            true
        };

        north_bound() || south_bound() || east_bound() || west_bound()
    };

    //maze.fancy_print(&visited);

    for y in 0..maze.tiles.len() {
        for x in 0..maze.tiles[0].len() {
            let point = Point2D { x, y };
            if !visited.contains(&point) {
                maze.tiles[y][x] = Tile::Empty;
            }
        }
    }

    for y in 0..maze.tiles.len() {
        for x in 0..maze.tiles[0].len() {
            let point = Point2D { x, y };
            if maze.get_tile_at(point) == Tile::Empty && outside_loop(point) {
                maze.poison_tile(point);
            }
        }
    }

    maze.fancy_print(&visited);

    maze.tiles
        .iter()
        .map(|row| row.iter().filter(|&tile| *tile == Tile::Empty).count() as i64)
        .sum()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Empty,
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Start,
    External,
}

impl Tile {
    const fn from_char(ch: char) -> Self {
        match ch {
            'S' => Self::Start,
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'L' => Self::NorthEast,
            'J' => Self::NorthWest,
            'F' => Self::SouthEast,
            '7' => Self::SouthWest,
            _ => Self::Empty,
        }
    }

    fn rotate(self, direction: Direction) -> Direction {
        match self {
            Self::Vertical | Self::Horizontal => direction,
            Self::NorthEast => {
                if direction == Direction::South {
                    Direction::East
                } else {
                    Direction::North
                }
            }
            Self::NorthWest => {
                if direction == Direction::South {
                    Direction::West
                } else {
                    Direction::North
                }
            }
            Self::SouthEast => {
                if direction == Direction::North {
                    Direction::East
                } else {
                    Direction::South
                }
            }
            Self::SouthWest => {
                if direction == Direction::North {
                    Direction::West
                } else {
                    Direction::South
                }
            }
            _ => unreachable!(),
        }
    }

    fn connects(self, direction: Direction) -> bool {
        match self {
            Self::Vertical => direction == Direction::North || direction == Direction::South,
            Self::Horizontal => direction == Direction::West || direction == Direction::East,
            Self::NorthEast => direction == Direction::South || direction == Direction::West,
            Self::NorthWest => direction == Direction::South || direction == Direction::East,
            Self::SouthEast => direction == Direction::North || direction == Direction::West,
            Self::SouthWest => direction == Direction::North || direction == Direction::East,
            Self::Start => true,
            Self::External | Self::Empty => false,
        }
    }

    fn to_char(self) -> char {
        match self {
            Tile::Empty => '.',
            Tile::Vertical => '|',
            Tile::Horizontal => '-',
            Tile::NorthEast => 'L',
            Tile::NorthWest => 'J',
            Tile::SouthEast => 'F',
            Tile::SouthWest => '7',
            Tile::Start => 'S',
            Tile::External => 'O',
        }
    }

    fn is_north(self) -> bool {
        self == Self::NorthEast || self == Self::NorthWest
    }

    fn is_south(self) -> bool {
        self == Self::SouthEast || self == Self::SouthWest
    }

    fn is_east(self) -> bool {
        self == Self::NorthEast || self == Self::SouthEast
    }

    fn is_west(self) -> bool {
        self == Self::NorthWest || self == Self::SouthWest
    }

    fn horizontal_gap(top: Self, bottom: Self) -> bool {
        (top.is_north() || top == Self::Horizontal || top == Self::Empty)
            && (bottom.is_south() || bottom == Self::Horizontal || bottom == Self::Empty)
    }

    fn vertical_gap(left: Self, right: Self) -> bool {
        (left.is_west() || left == Self::Vertical || left == Self::Empty)
            && (right.is_east() || right == Self::Vertical || right == Self::Empty)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
struct Point2D {
    x: usize,
    y: usize,
}

impl Point2D {
    const fn get_next(&self, direction: Direction) -> Self {
        match direction {
            Direction::North => Self {
                x: self.x,
                y: self.y - 1,
            },
            Direction::South => Self {
                x: self.x,
                y: self.y + 1,
            },
            Direction::East => Self {
                x: self.x + 1,
                y: self.y,
            },
            Direction::West => Self {
                x: self.x - 1,
                y: self.y,
            },
        }
    }
}

struct Cursor {
    point: Point2D,
    direction: Direction,
}

#[derive(Debug, PartialEq, Eq)]
struct Maze {
    tiles: Vec<Vec<Tile>>,
    start: Point2D,
}

impl Maze {
    fn get_next(&self, cursor: &Cursor) -> Cursor {
        let tile = self.tiles[cursor.point.y][cursor.point.x];

        let next_direction = tile.rotate(cursor.direction);

        Cursor {
            point: cursor.point.get_next(next_direction),
            direction: next_direction,
        }
    }

    fn init_start(&mut self) {
        let [dir1, dir2] = self.get_from_start();

        self.tiles[self.start.y][self.start.x] = match dir1 {
            Direction::North => match dir2 {
                Direction::North | Direction::South => Tile::Vertical,
                Direction::East => Tile::NorthEast,
                Direction::West => Tile::NorthWest,
            },
            Direction::South => match dir2 {
                Direction::North | Direction::South => Tile::Vertical,
                Direction::East => Tile::SouthEast,
                Direction::West => Tile::SouthWest,
            },
            Direction::East => match dir2 {
                Direction::North => Tile::NorthEast,
                Direction::South => Tile::SouthEast,
                Direction::East | Direction::West => Tile::Horizontal,
            },
            Direction::West => match dir2 {
                Direction::North => Tile::NorthWest,
                Direction::South => Tile::SouthWest,
                Direction::East | Direction::West => Tile::Horizontal,
            },
        };
    }

    fn get_from_start(&self) -> [Direction; 2] {
        let mut directions = [Direction::North, Direction::North];
        let mut index = 0;

        if self.start.y != 0
            && self.tiles[self.start.y - 1][self.start.x].connects(Direction::North)
        {
            directions[index] = Direction::North;
            index += 1;
        }

        if self.start.y != self.tiles.len() - 1
            && self.tiles[self.start.y + 1][self.start.x].connects(Direction::South)
        {
            directions[index] = Direction::South;
            index += 1;
        }

        if self.start.x != 0 && self.tiles[self.start.y][self.start.x - 1].connects(Direction::West)
        {
            directions[index] = Direction::West;
            index += 1;
        }

        if self.start.x != self.tiles[0].len() - 1
            && self.tiles[self.start.y][self.start.x + 1].connects(Direction::East)
        {
            directions[index] = Direction::East;
        }

        directions
    }

    fn get_tile_at(&self, point: Point2D) -> Tile {
        if point.y >= self.tiles.len() || point.x >= self.tiles[0].len() {
            Tile::External
        } else {
            self.tiles[point.y][point.x]
        }
    }

    fn get_next_tile(&self, cursor: &Cursor) -> Tile {
        let is_edge = match cursor.direction {
            Direction::North => cursor.point.y == 0,
            Direction::South => cursor.point.y == self.tiles.len() - 1,
            Direction::East => cursor.point.x == self.tiles[0].len() - 1,
            Direction::West => cursor.point.x == 0,
        };

        if is_edge {
            Tile::External
        } else {
            self.get_tile_at(cursor.point.get_next(cursor.direction))
        }
    }

    fn poison_tile_dir(&mut self, point: Point2D, direction: Direction) {
        let shared = RefCell::new(self);

        let is_gap = |tile: Tile, cursor: &Cursor| match cursor.direction {
            Direction::North | Direction::South => Tile::vertical_gap(
                tile,
                shared
                    .borrow()
                    .get_tile_at(cursor.point.get_next(Direction::East)),
            ),
            Direction::East | Direction::West => Tile::horizontal_gap(
                tile,
                shared
                    .borrow()
                    .get_tile_at(cursor.point.get_next(Direction::South)),
            ),
        };

        let mut cursor = Cursor { point, direction };

        if shared.borrow().get_tile_at(point) == Tile::Empty {
            shared.borrow_mut().tiles[point.y][point.x] = Tile::External;
        }

        let mut next_tile = shared.borrow().get_next_tile(&cursor);

        if next_tile == Tile::External {
            return;
        }

        cursor.point = cursor.point.get_next(cursor.direction);

        while next_tile == Tile::Empty || is_gap(next_tile, &cursor) {
            for direction in [
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West,
            ] {
                // TODO: Remove recursion to avoid stack overflow
                shared.borrow_mut().poison_tile_dir(cursor.point, direction);
            }

            next_tile = shared.borrow().get_next_tile(&cursor);

            if next_tile == Tile::External {
                break;
            }
            cursor.point = cursor.point.get_next(cursor.direction);
        }
    }

    fn poison_tile(&mut self, point: Point2D) {
        for direction in [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ] {
            self.poison_tile_dir(point, direction);
        }
    }

    fn fancy_print(&self, visited: &HashSet<Point2D>) {
        for y in 0..self.tiles.len() {
            for x in 0..self.tiles[y].len() {
                let point = Point2D { x, y };
                if visited.contains(&point) {
                    print!("{}", self.get_tile_at(point).to_char());
                } else {
                    print!("\x1b[91m{}\x1b[0m", self.get_tile_at(point).to_char());
                }
            }

            println!();
        }

        println!();
    }
}

struct PoisonIter {
    stack: Vec<(Cursor, u32)>,
    max_depth: u32,
}

impl PoisonIter {
    fn new(root: Cursor, max_depth: u32) -> Self {
        Self {
            stack: vec![(root, 0)],
            max_depth,
        }
    }
}

impl Iterator for PoisonIter {
    type Item = Point2D;

    fn next(&mut self) -> Option<Self::Item> {
        let (state, depth) = self.stack.pop()?;

        if depth < self.max_depth {
            for direction in [
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West,
            ] {
                self.stack.push((
                    Cursor {
                        point: state.point.get_next(direction),
                        direction,
                    },
                    depth + 1,
                ));
            }
        }

        Some(state.point)
    }
}

fn parse_input(input: &[String]) -> Maze {
    let mut start = Point2D::default();

    let tiles = input
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, ch)| {
                    if ch == 'S' {
                        start = Point2D { x, y };
                    }

                    Tile::from_char(ch)
                })
                .collect()
        })
        .collect();

    let mut maze = Maze { tiles, start };
    maze.init_start();

    maze
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input_a() -> [String; 5] {
        [
            ".....".to_owned(),
            ".S-7.".to_owned(),
            ".|.|.".to_owned(),
            ".L-J.".to_owned(),
            ".....".to_owned(),
        ]
    }

    fn get_test_output_a() -> Maze {
        Maze {
            tiles: vec![
                vec![
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Empty,
                ],
                vec![
                    Tile::Empty,
                    Tile::SouthEast,
                    Tile::Horizontal,
                    Tile::SouthWest,
                    Tile::Empty,
                ],
                vec![
                    Tile::Empty,
                    Tile::Vertical,
                    Tile::Empty,
                    Tile::Vertical,
                    Tile::Empty,
                ],
                vec![
                    Tile::Empty,
                    Tile::NorthEast,
                    Tile::Horizontal,
                    Tile::NorthWest,
                    Tile::Empty,
                ],
                vec![
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Empty,
                    Tile::Empty,
                ],
            ],
            start: Point2D { x: 1, y: 1 },
        }
    }

    fn get_test_input_b() -> [String; 5] {
        [
            "..F7.".to_owned(),
            ".FJ|.".to_owned(),
            "SJ.L7".to_owned(),
            "|F--J".to_owned(),
            "LJ...".to_owned(),
        ]
    }

    fn get_test_input_c1() -> [String; 9] {
        [
            "...........".to_owned(),
            ".S-------7.".to_owned(),
            ".|F-----7|.".to_owned(),
            ".||.....||.".to_owned(),
            ".||.....||.".to_owned(),
            ".|L-7.F-J|.".to_owned(),
            ".|..|.|..|.".to_owned(),
            ".L--J.L--J.".to_owned(),
            "...........".to_owned(),
        ]
    }

    fn get_test_input_c2() -> [String; 9] {
        [
            "..........".to_owned(),
            ".S------7.".to_owned(),
            ".|F----7|.".to_owned(),
            ".||....||.".to_owned(),
            ".||....||.".to_owned(),
            ".|L-7F-J|.".to_owned(),
            ".|..||..|.".to_owned(),
            ".L--JL--J.".to_owned(),
            "..........".to_owned(),
        ]
    }

    fn get_test_input_d() -> [String; 10] {
        todo!()
    }

    fn get_test_input_e() -> [String; 10] {
        todo!()
    }

    #[test]
    fn parse_input_test_a() {
        let result = parse_input(&get_test_input_a());
        let expected_result = get_test_output_a();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn poison_tile_test() {
        let mut maze = parse_input(&get_test_input_a());
        maze.poison_tile(Point2D { x: 0, y: 0 });

        for tile in &maze.tiles[0] {
            assert_eq!(*tile, Tile::External);
        }

        for y in 1..=3 {
            let tile0 = maze.tiles[y][0];
            let tile1 = maze.tiles[y][4];

            assert_eq!(tile0, Tile::External);
            assert_eq!(tile1, Tile::External);
        }

        for tile in &maze.tiles[4] {
            assert_eq!(*tile, Tile::External);
        }
    }

    #[test]
    fn part1_ex_test_a() {
        let result = part1(&get_test_input_a());

        assert_eq!(result, 4);
    }

    #[test]
    fn part1_ex_test_b() {
        let result = part1(&get_test_input_b());

        assert_eq!(result, 8);
    }

    #[test]
    fn part2_ex_test_a() {
        let result = part2(&get_test_input_a());

        assert_eq!(result, 1);
    }

    #[test]
    fn part2_ex_test_b() {
        let result = part2(&get_test_input_b());

        assert_eq!(result, 1);
    }

    // #[test]
    // fn part2_ex_test_c() {
    //     let result1 = part2(&get_test_input_c1());
    //     assert_eq!(result1, 4);

    //     let result2 = part2(&get_test_input_c2());
    //     assert_eq!(result2, 4);
    // }

    // #[test]
    // fn part2_ex_test_d() {
    //     let result = part2(&get_test_input_d());

    //     assert_eq!(result, 8);
    // }

    // #[test]
    // fn part2_ex_test_e() {
    //     let result = part2(&get_test_input_e());

    //     assert_eq!(result, 10);
    // }
}
