use std::ops::{Index, IndexMut};

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let map = Map(input);
    let start = map.find_start();

    let mut probes = spawn_probes(start, map);
    let mut steps = 1;

    while probes.0.position != probes.1.position {
        probes.0.advance(map);
        probes.1.advance(map);
        steps += 1;
    }

    Ok(steps)
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let map = Map(input);
    let start = map.find_start();

    let mut probes = spawn_probes(start, map);
    let mut pipe_map = PipeMap::new(map);
    pipe_map[start] = find_pipe((probes.0.direction, probes.1.direction));

    while probes.0.position != probes.1.position {
        pipe_map[probes.0.position] = map[probes.0.position];
        pipe_map[probes.1.position] = map[probes.1.position];
        probes.0.advance(map);
        probes.1.advance(map);
    }

    pipe_map[probes.0.position] = map[probes.0.position];
    Ok(pipe_map.inside_area())
}

fn spawn_probes(start: Position, map: Map) -> (Probe, Probe) {
    let candidates = [
        Probe {
            position: Position::new(start.x, start.y - 1),
            direction: Direction::Up,
        },
        Probe {
            position: Position::new(start.x, start.y + 1),
            direction: Direction::Down,
        },
        Probe {
            position: Position::new(start.x - 1, start.y),
            direction: Direction::Left,
        },
        Probe {
            position: Position::new(start.x + 1, start.y),
            direction: Direction::Right,
        },
    ];

    let mut probes = candidates.into_iter().filter(|probe| {
        let pipe = map[probe.position];

        match pipe {
            b'|' => matches!(probe.direction, Direction::Up | Direction::Down),
            b'-' => matches!(probe.direction, Direction::Left | Direction::Right),
            b'L' => matches!(probe.direction, Direction::Down | Direction::Left),
            b'J' => matches!(probe.direction, Direction::Down | Direction::Right),
            b'7' => matches!(probe.direction, Direction::Up | Direction::Right),
            b'F' => matches!(probe.direction, Direction::Up | Direction::Left),
            _ => false,
        }
    });

    (probes.next().unwrap(), probes.next().unwrap())
}

fn find_pipe(directions: (Direction, Direction)) -> u8 {
    match directions {
        (Direction::Up, Direction::Down) | (Direction::Down, Direction::Up) => b'|',
        (Direction::Up, Direction::Left) | (Direction::Left, Direction::Up) => b'J',
        (Direction::Up, Direction::Right) | (Direction::Right, Direction::Up) => b'L',
        (Direction::Down, Direction::Left) | (Direction::Left, Direction::Down) => b'7',
        (Direction::Down, Direction::Right) | (Direction::Right, Direction::Down) => b'F',
        (Direction::Left, Direction::Right) | (Direction::Right, Direction::Left) => b'-',
        _ => unreachable!(),
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

struct Probe {
    position: Position,
    direction: Direction,
}

impl Probe {
    fn advance(&mut self, map: Map) {
        match (self.direction, map[self.position]) {
            (Direction::Up, b'|') => {
                self.position.y -= 1;
            }
            (Direction::Up, b'7') => {
                self.position.x -= 1;
                self.direction = Direction::Left;
            }
            (Direction::Up, b'F') => {
                self.position.x += 1;
                self.direction = Direction::Right;
            }
            (Direction::Down, b'|') => {
                self.position.y += 1;
            }
            (Direction::Down, b'L') => {
                self.position.x += 1;
                self.direction = Direction::Right;
            }
            (Direction::Down, b'J') => {
                self.position.x -= 1;
                self.direction = Direction::Left;
            }
            (Direction::Left, b'-') => {
                self.position.x -= 1;
            }
            (Direction::Left, b'L') => {
                self.position.y -= 1;
                self.direction = Direction::Up;
            }
            (Direction::Left, b'F') => {
                self.position.y += 1;
                self.direction = Direction::Down;
            }
            (Direction::Right, b'-') => {
                self.position.x += 1;
            }
            (Direction::Right, b'J') => {
                self.position.y -= 1;
                self.direction = Direction::Up;
            }
            (Direction::Right, b'7') => {
                self.position.y += 1;
                self.direction = Direction::Down;
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone)]
struct Map<'a>(&'a [&'a str]);

impl Map<'_> {
    fn find_start(&self) -> Position {
        self.0
            .iter()
            .enumerate()
            .find_map(|(y, line)| {
                line.bytes()
                    .position(|tile| tile == b'S')
                    .map(|x| Position::new(x, y))
            })
            .unwrap()
    }
}

impl Index<Position> for Map<'_> {
    type Output = u8;

    fn index(&self, index: Position) -> &Self::Output {
        &self.0[index.y].as_bytes()[index.x]
    }
}

struct PipeMap {
    width: usize,
    tiles: Vec<u8>,
}

impl PipeMap {
    fn new(map: Map) -> Self {
        let width = map.0[0].len();
        let tiles = vec![b'.'; map.0.len() * width];
        Self { width, tiles }
    }

    fn inside_area(&self) -> usize {
        let mut count = 0;

        #[derive(Copy, Clone)]
        enum State {
            None,
            UpPipe,
            DownPipe,
        }

        for row in self.tiles.chunks_exact(self.width) {
            let mut inside = false;
            let mut state = State::None;

            for tile in row {
                // We're inside the loop whenever we've passed over an odd number of horizontal pipes.
                // That's trivial for '|' pipes but slightly harder 90-degree pipes.
                // We can only encounter 2 of them outside the loop: 'L' and 'F'.
                // Whether we've passed a horizontal pipe depends on the direction of the next pipe.
                // If the first one turned upwards, the next one must turn downwards, and vice-versa.
                // Examples:
                // pipe: "|"     horizontal: true
                // pipe: "L7"    horizontal: true
                // pipe: "LJ"    horizontal: false
                // pipe: "FJ"    horizontal: true
                // pipe: "F7"    horizontal: false
                // pipe: "L--7"  horizontal: true
                // pipe: "L--J"  horizontal: false
                match (state, tile) {
                    (State::None, b'.') if inside => count += 1,
                    (State::None, b'|') => inside = !inside,
                    (State::None, b'L') => state = State::UpPipe,
                    (State::None, b'F') => state = State::DownPipe,
                    (State::UpPipe, b'J') | (State::DownPipe, b'7') => state = State::None,
                    (State::UpPipe, b'7') | (State::DownPipe, b'J') => {
                        state = State::None;
                        inside = !inside;
                    }
                    _ => {}
                }
            }
        }

        count
    }
}

impl Index<Position> for PipeMap {
    type Output = u8;

    fn index(&self, index: Position) -> &Self::Output {
        &self.tiles[index.x + index.y * self.width]
    }
}

impl IndexMut<Position> for PipeMap {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        &mut self.tiles[index.x + index.y * self.width]
    }
}
