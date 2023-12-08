use gcd::Gcd;
use std::ops::Range;

const START_RANGE: Range<usize> = 0..3;
const LEFT_RANGE: Range<usize> = 7..10;
const RIGHT_RANGE: Range<usize> = 12..15;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut network = Network::default();

    for line in &input[2..] {
        let (start, left, right) = parse_network_entry(line);
        network.insert(start, left, right);
    }

    let directions = parse_directions(input[0]);
    Ok(count_steps(Node::default(), &network, directions))
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut positions = Vec::new();
    let mut network = Network::default();

    for line in &input[2..] {
        let (start, left, right) = parse_network_entry(line);
        network.insert(start, left, right);

        if start.0 % 26 == 0 {
            positions.push(start);
        }
    }

    let directions = parse_directions(input[0]);

    let solution = positions.into_iter().fold(1u64, |solution, position| {
        // The paths taken from each starting position seem to form a cycle.
        // They don't actually go over the same nodes, but they do always end up on the exact same Z node.
        // And the number of steps from the start node to the first Z node and from one Z node to the next
        // is always the same.
        // So from starting position 1 we'll get to a Z node every X steps, and from position 2 every Y steps, ..
        // This means that we'll visit a Z node from all starting positions simulatniously on the first step
        // divisible by all of the cycle lengths.
        let steps = count_steps(position, &network, directions.clone()) as u64;
        solution * steps / solution.gcd(steps)
    });

    Ok(solution)
}

fn parse_network_entry(value: &str) -> (Node, Node, Node) {
    let start = Node::from(&value[START_RANGE]);
    let left = Node::from(&value[LEFT_RANGE]);
    let right = Node::from(&value[RIGHT_RANGE]);
    (start, left, right)
}

fn parse_directions(value: &str) -> impl Iterator<Item = Direction> + Clone + '_ {
    value.bytes().map(Direction::from).cycle()
}

fn count_steps(
    mut position: Node,
    network: &Network,
    directions: impl Iterator<Item = Direction>,
) -> usize {
    let mut steps = 0;

    for direction in directions {
        steps += 1;
        position = network.next_position(position, direction);

        if position.0 % 26 == 25 {
            break;
        }
    }

    steps
}

enum Direction {
    Left,
    Right,
}

impl From<u8> for Direction {
    fn from(value: u8) -> Self {
        match value {
            b'L' => Self::Left,
            b'R' => Self::Right,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone, Default)]
struct Node(u16);

impl From<&str> for Node {
    fn from(value: &str) -> Self {
        let value = value
            .bytes()
            .fold(0, |acc, char| (acc * 26) + (char - b'A') as u16);

        Self(value)
    }
}

struct Network([(Node, Node); 17576]);

impl Network {
    fn insert(&mut self, start: Node, left: Node, right: Node) {
        self.0[start.0 as usize] = (left, right);
    }

    fn next_position(&self, current: Node, direction: Direction) -> Node {
        match direction {
            Direction::Left => self.0[current.0 as usize].0,
            Direction::Right => self.0[current.0 as usize].1,
        }
    }
}

impl Default for Network {
    fn default() -> Self {
        Self([(Node::default(), Node::default()); 17576])
    }
}
