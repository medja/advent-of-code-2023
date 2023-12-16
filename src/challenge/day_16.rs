pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut grid = Grid::new(input);
    grid.fire_beam((0, 0), Direction::RIGHT);
    Ok(grid.energized)
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut grid = Grid::new(input);
    let initial_cells = grid.cells.clone();
    let mut best = 0;

    for x in 0..grid.width {
        grid.reset(&initial_cells);
        grid.fire_beam((x, 0), Direction::DOWN);
        best = best.max(grid.energized);

        grid.reset(&initial_cells);
        grid.fire_beam((x, grid.max_y), Direction::UP);
        best = best.max(grid.energized);
    }

    for y in 0..grid.height {
        grid.reset(&initial_cells);
        grid.fire_beam((0, y), Direction::RIGHT);
        best = best.max(grid.energized);

        grid.reset(&initial_cells);
        grid.fire_beam((grid.max_x, y), Direction::LEFT);
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
            Tile::VerticalSplit if matches!(direction, Direction::LEFT | Direction::RIGHT) => {
                if let Some(position) = self.next_position(position, Direction::UP) {
                    self.fire_beam(position, Direction::UP)
                }

                Direction::DOWN
            }
            Tile::HorizontalSplit if matches!(direction, Direction::UP | Direction::DOWN) => {
                if let Some(position) = self.next_position(position, Direction::LEFT) {
                    self.fire_beam(position, Direction::LEFT)
                }

                Direction::RIGHT
            }
            Tile::NormalMirror => direction.normal_mirror(),
            Tile::ReverseMirror => direction.reverse_mirror(),
            _ => direction,
        }
    }

    fn next_position(
        &self,
        (x, y): (usize, usize),
        direction: Direction,
    ) -> Option<(usize, usize)> {
        match direction {
            Direction::UP if y > 0 => Some((x, y - 1)),
            Direction::DOWN if y < self.max_y => Some((x, y + 1)),
            Direction::LEFT if x > 0 => Some((x - 1, y)),
            Direction::RIGHT if x < self.max_x => Some((x + 1, y)),
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

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
struct Direction(u8);

impl Direction {
    const UP: Self = Direction(0);
    const DOWN: Self = Direction(1);
    const LEFT: Self = Direction(2);
    const RIGHT: Self = Direction(3);

    fn to_flag(self) -> u8 {
        1 << self.0
    }

    fn normal_mirror(self) -> Self {
        Self(3 - self.0)
    }

    fn reverse_mirror(self) -> Self {
        Self((self.0 + 2) % 4)
    }
}
