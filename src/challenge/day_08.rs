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
        // The paths taken from each starting position form a cycle.
        // And each path only ever visits a single Z node.
        // The number of steps required to reach the Z node is constant across all iterations.
        // (That number is also divisible by the number of directions)
        // The first few nodes aren't part of the cycle, but the number of steps required to
        // reach the first Z node is identical to later iterations.
        // This means that we reach a Z node every N steps, where N is different for every path.
        // And we're only ever on a Z node on the Nth step.
        // As a result, the number of steps at which we're on all Z nodes at once must be
        // divisible by all of the cycle lengths. And the fist time that happens is on the
        // smallest (least) common multiple of all of the cycle lengths.
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
