pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut grid = Grid::new(input);
    grid.fire_beam((0, 0), Direction::Right);
    Ok(grid.energized)
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut grid = Grid::new(input);
    let initial_cells = grid.cells.clone();
    let mut best = 0;

    for x in 0..grid.width {
        grid.reset(&initial_cells);
        grid.fire_beam((x, 0), Direction::Down);
        best = best.max(grid.energized);

        grid.reset(&initial_cells);
        grid.fire_beam((x, grid.max_y), Direction::Up);
        best = best.max(grid.energized);
    }

    for y in 0..grid.height {
        grid.reset(&initial_cells);
        grid.fire_beam((0, y), Direction::Right);
        best = best.max(grid.energized);

        grid.reset(&initial_cells);
        grid.fire_beam((grid.max_x, y), Direction::Left);
        best = best.max(grid.energized);
    }

    Ok(best)
}

struct Grid {
    width: usize,
    height: usize,
    max_x: usize,
    max_y: usize,
    cells: Vec<Cell>,
    energized: usize,
}

impl Grid {
    fn new(input: &[&str]) -> Self {
        let width = input[0].len();
        let height = input.len();

        let cells = input
            .iter()
            .flat_map(|row| row.bytes())
            .map(Cell::from)
            .collect();

        Self {
            width,
            height,
            max_x: width - 1,
            max_y: height - 1,
            cells,
            energized: 0,
        }
    }

    fn fire_beam(&mut self, mut position: (usize, usize), mut direction: Direction) {
        loop {
            let cell = &mut self.cells[position.0 + position.1 * self.width];
            let tile = cell.tile;
            let flag = direction.to_flag();

            if cell.state == 0 {
                cell.state = flag;
                self.energized += 1;
            } else if cell.state & flag == 0 {
                cell.state |= flag;
            } else {
                break;
            }

            direction = self.simulate_beam(position, direction, tile);

            position = match self.next_position(position, direction) {
                Some(position) => position,
                None => break,
            }
        }
    }

    fn simulate_beam(
        &mut self,
        position: (usize, usize),
        direction: Direction,
        tile: Tile,
    ) -> Direction {
        match tile {
            Tile::VerticalSplit if matches!(direction, Direction::Left | Direction::Right) => {
                if let Some(position) = self.next_position(position, Direction::Up) {
                    self.fire_beam(position, Direction::Up)
                }

                Direction::Down
            }
            Tile::HorizontalSplit if matches!(direction, Direction::Up | Direction::Down) => {
                if let Some(position) = self.next_position(position, Direction::Left) {
                    self.fire_beam(position, Direction::Left)
                }

                Direction::Right
            }
            Tile::NormalMirror => match direction {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up,
            },
            Tile::ReverseMirror => match direction {
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            },
            _ => direction,
        }
    }

    fn next_position(
        &self,
        (x, y): (usize, usize),
        direction: Direction,
    ) -> Option<(usize, usize)> {
        match direction {
            Direction::Up if y > 0 => Some((x, y - 1)),
            Direction::Down if y < self.max_y => Some((x, y + 1)),
            Direction::Left if x > 0 => Some((x - 1, y)),
            Direction::Right if x < self.max_x => Some((x + 1, y)),
            _ => None,
        }
    }

    fn reset(&mut self, initial_cells: &[Cell]) {
        self.cells.copy_from_slice(initial_cells);
        self.energized = 0;
    }
}

#[derive(Copy, Clone)]
struct Cell {
    tile: Tile,
    state: u8,
}

impl From<u8> for Cell {
    fn from(value: u8) -> Self {
        let tile = match value {
            b'.' => Tile::Empty,
            b'|' => Tile::VerticalSplit,
            b'-' => Tile::HorizontalSplit,
            b'/' => Tile::NormalMirror,
            b'\\' => Tile::ReverseMirror,
            _ => unreachable!(),
        };

        Self { tile, state: 0 }
    }
}

#[derive(Copy, Clone)]
enum Tile {
    Empty,           // .
    VerticalSplit,   // |
    HorizontalSplit, // -
    NormalMirror,    // /
    ReverseMirror,   // \
}

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn to_flag(self) -> u8 {
        match self {
            Direction::Up => 1,
            Direction::Down => 2,
            Direction::Left => 4,
            Direction::Right => 8,
        }
    }
}
