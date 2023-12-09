use std::{collections::HashMap, fmt::Display};

#[must_use]
pub fn part1(input: &[String]) -> u64 {
    const TERM_NODE: Node = Node::new(*b"ZZZ");

    let network = parse_input(input);

    count_steps(&network, Node::new(*b"AAA"), |node| node == TERM_NODE)
}

#[must_use]
pub fn part2(input: &[String]) -> u64 {
    let network = parse_input(input);

    let counts: Vec<u64> = network
        .nodes
        .map
        .keys()
        .filter(|node| node.id[2] == b'A')
        .map(|node| count_steps(&network, *node, |n| n.id[2] == b'Z'))
        .collect();

    lcm(&counts)
}

fn count_steps(network: &Network, start: Node, pred: fn(Node) -> bool) -> u64 {
    let mut current_node = start;

    for step in 0.. {
        if pred(current_node) {
            return step as u64;
        }

        let instruction = network.get_instruction(step);

        current_node = match instruction {
            Direction::Left => network.nodes.left(current_node).unwrap(),
            Direction::Right => network.nodes.right(current_node).unwrap(),
        };
    }

    unreachable!()
}

fn lcm(nums: &[u64]) -> u64 {
    if nums.len() == 1 {
        return nums[0];
    }

    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd(a, b)
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }

    gcd(b, a % b)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Direction {
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct Node {
    id: [u8; 3],
}

impl Node {
    const fn new(id: [u8; 3]) -> Self {
        Self { id }
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
struct NodeMap {
    map: HashMap<Node, (Node, Node)>,
}

impl NodeMap {
    fn insert(&mut self, node: Node, left: Node, right: Node) {
        self.map.insert(node, (left, right));
    }

    fn insert_from_str(&mut self, node: [u8; 3], left: [u8; 3], right: [u8; 3]) {
        self.insert(Node::new(node), Node::new(left), Node::new(right));
    }

    fn left(&self, node: Node) -> Option<Node> {
        self.map.get(&node).map(|x| x.0)
    }

    fn right(&self, node: Node) -> Option<Node> {
        self.map.get(&node).map(|x| x.1)
    }
}

impl Display for NodeMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{{")?;

        for (k, v) in &self.map {
            writeln!(f, "  {:?} = ({:?}, {:?})", k.id, v.0.id, v.1.id)?;
        }

        write!(f, "}}")
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Network {
    instructions: Vec<Direction>,
    nodes: NodeMap,
}

impl Network {
    fn get_instruction(&self, index: usize) -> Direction {
        let fixed = index % self.instructions.len();

        self.instructions[fixed]
    }
}

fn parse_input(input: &[String]) -> Network {
    let instructions = input[0]
        .chars()
        .filter_map(|c| match c {
            'L' => Some(Direction::Left),
            'R' => Some(Direction::Right),
            _ => None,
        })
        .collect();

    let mut nodes = NodeMap::default();

    input.iter().skip(2).for_each(|line| {
        let mut letters = line.bytes().filter(u8::is_ascii_alphanumeric);

        let s1 = [(); 3].map(|()| letters.next().unwrap());
        let s2 = [(); 3].map(|()| letters.next().unwrap());
        let s3 = [(); 3].map(|()| letters.next().unwrap());

        nodes.insert_from_str(s1, s2, s3);
    });

    Network {
        instructions,
        nodes,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::manual_string_new)]
    fn get_test_input1_a() -> [String; 9] {
        [
            "RL".to_owned(),
            "".to_owned(),
            "AAA = (BBB, CCC)".to_owned(),
            "BBB = (DDD, EEE)".to_owned(),
            "CCC = (ZZZ, GGG)".to_owned(),
            "DDD = (DDD, DDD)".to_owned(),
            "EEE = (EEE, EEE)".to_owned(),
            "GGG = (GGG, GGG)".to_owned(),
            "ZZZ = (ZZZ, ZZZ)".to_owned(),
        ]
    }

    fn get_test_output1_a() -> Network {
        let mut node_map = NodeMap::default();

        node_map.insert_from_str(*b"AAA", *b"BBB", *b"CCC");
        node_map.insert_from_str(*b"BBB", *b"DDD", *b"EEE");
        node_map.insert_from_str(*b"CCC", *b"ZZZ", *b"GGG");
        node_map.insert_from_str(*b"DDD", *b"DDD", *b"DDD");
        node_map.insert_from_str(*b"EEE", *b"EEE", *b"EEE");
        node_map.insert_from_str(*b"GGG", *b"GGG", *b"GGG");
        node_map.insert_from_str(*b"ZZZ", *b"ZZZ", *b"ZZZ");

        Network {
            instructions: vec![Direction::Right, Direction::Left],
            nodes: node_map,
        }
    }

    #[allow(clippy::manual_string_new)]
    fn get_test_input1_b() -> [String; 5] {
        [
            "LLR".to_owned(),
            "".to_owned(),
            "AAA = (BBB, BBB)".to_owned(),
            "BBB = (AAA, ZZZ)".to_owned(),
            "ZZZ = (ZZZ, ZZZ)".to_owned(),
        ]
    }

    fn get_test_output1_b() -> Network {
        let mut node_map = NodeMap::default();

        node_map.insert_from_str(*b"AAA", *b"BBB", *b"BBB");
        node_map.insert_from_str(*b"BBB", *b"AAA", *b"ZZZ");
        node_map.insert_from_str(*b"ZZZ", *b"ZZZ", *b"ZZZ");

        Network {
            instructions: vec![Direction::Left, Direction::Left, Direction::Right],
            nodes: node_map,
        }
    }

    #[allow(clippy::manual_string_new)]
    fn get_test_input2() -> [String; 10] {
        [
            "LR".to_owned(),
            "".to_owned(),
            "11A = (11B, XXX)".to_owned(),
            "11B = (XXX, 11Z)".to_owned(),
            "11Z = (11B, XXX)".to_owned(),
            "22A = (22B, XXX)".to_owned(),
            "22B = (22C, 22C)".to_owned(),
            "22C = (22Z, 22Z)".to_owned(),
            "22Z = (22B, 22B)".to_owned(),
            "XXX = (XXX, XXX)".to_owned(),
        ]
    }

    fn get_test_output2() -> Network {
        let mut node_map = NodeMap::default();

        node_map.insert_from_str(*b"11A", *b"11B", *b"XXX");
        node_map.insert_from_str(*b"11B", *b"XXX", *b"11Z");
        node_map.insert_from_str(*b"11Z", *b"11B", *b"XXX");
        node_map.insert_from_str(*b"22A", *b"22B", *b"XXX");
        node_map.insert_from_str(*b"22B", *b"22C", *b"22C");
        node_map.insert_from_str(*b"22C", *b"22Z", *b"22Z");
        node_map.insert_from_str(*b"22Z", *b"22B", *b"22B");
        node_map.insert_from_str(*b"XXX", *b"XXX", *b"XXX");

        Network {
            instructions: vec![Direction::Left, Direction::Right],
            nodes: node_map,
        }
    }

    #[test]
    fn parse_input_test() {
        let result1_a = parse_input(&get_test_input1_a());
        let expected_result1_a = get_test_output1_a();

        assert_eq!(result1_a, expected_result1_a);

        let result1_b = parse_input(&get_test_input1_b());
        let expected_result1_b = get_test_output1_b();

        assert_eq!(result1_b, expected_result1_b);

        let result2 = parse_input(&get_test_input2());
        let expected_result2 = get_test_output2();

        assert_eq!(result2, expected_result2);
    }

    #[test]
    fn part1_ex_test() {
        let result_a = part1(&get_test_input1_a());

        assert_eq!(result_a, 2);

        let result_b = part1(&get_test_input1_b());

        assert_eq!(result_b, 6);
    }

    #[test]
    fn part2_ex_test() {
        let result = part2(&get_test_input2());

        assert_eq!(result, 6);
    }
}
