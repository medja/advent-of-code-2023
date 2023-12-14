use std::ops::{Index, IndexMut};

const CYCLE_COUNT: usize = 1000000000;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut total_load = 0;

    for x in 0..input[0].len() {
        let mut load = input.len();

        for (y, row) in input.iter().enumerate() {
            match row.as_bytes()[x] {
                b'#' => {
                    load = input.len() - y - 1;
                }
                b'O' => {
                    total_load += load;
                    load -= 1;
                }
                _ => {}
            }
        }
    }

    Ok(total_load)
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut grid = Grid::new(input);

    // There aren't that many possible arangements, so the layouts start to loop.
    // Coincidently the loop can be observed by looking at the total loads on the north beam.
    let mut loads = Vec::with_capacity(200);

    // Blindly compute a few cycles as the first few are unlikely to contain a loop.
    // If we compute to many, we'll end up wasting time as simulating the cycle is very expensive.
    loads.extend((0..150).map(|_| grid.cycle()));

    let position = loop {
        match find_loop(&loads) {
            Some(position) => break position,
            None => loads.push(grid.cycle()),
        }
    };

    let loop_length = loads.len() - position;
    let loop_offset = (CYCLE_COUNT - loads.len()) % loop_length;
    Ok(loads[position + loop_offset - 1])
}

fn find_loop(values: &[usize]) -> Option<usize> {
    let last = *values.last().unwrap();

    // Iterate over the values in reverse, skipping the latest one.
    // If `x` is the current value and `z` is the latest value, check if `a, b, c, ...` are the same.
    // [..., a, b, c, d, e, x, a, b, c, d, e, z]
    // If they are, assume we found a loop and return the current position.
    values
        .iter()
        .enumerate()
        .rev()
        .skip(1)
        .filter(|(_, value)| **value == last)
        .map(|(i, _)| i + 1)
        .find(|&i| {
            // i is the potnetial position of the start of the next iteration of the loop
            let first = &values[i..values.len() - 1];
            let second = &values[2 * i - values.len()..i - 1];
            // first and second contain the current and next instance of the loop
            first == second
        })
}

struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
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
            cells,
        }
    }

    fn cycle(&mut self) -> usize {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
        self.total_load()
    }

    fn tilt_north(&mut self) {
        self.tilt(
            0..self.width,
            0..self.height,
            |position| position + 1,
            |this, x, y| this[(x, y)],
            |this, x, y, value| this[(x, y)] = value,
        )
    }

    fn tilt_west(&mut self) {
        self.tilt(
            0..self.height,
            0..self.width,
            |position| position + 1,
            |this, y, x| this[(x, y)],
            |this, y, x, value| this[(x, y)] = value,
        )
    }

    fn tilt_south(&mut self) {
        self.tilt(
            0..self.width,
            (0..self.height).rev(),
            |position| position.saturating_sub(1),
            |this, x, y| this[(x, y)],
            |this, x, y, value| this[(x, y)] = value,
        )
    }

    fn tilt_east(&mut self) {
        self.tilt(
            0..self.height,
            (0..self.width).rev(),
            |position| position.saturating_sub(1),
            |this, y, x| this[(x, y)],
            |this, y, x, value| this[(x, y)] = value,
        )
    }

    fn tilt(
        &mut self,
        lines: impl Iterator<Item = usize>,
        positions: impl Iterator<Item = usize> + Clone,
        next: impl Fn(usize) -> usize,
        get_cell: impl Fn(&Grid, usize, usize) -> Cell,
        set_cell: impl Fn(&mut Grid, usize, usize, Cell),
    ) {
        let start = positions.clone().next().unwrap();

        for line in lines {
            let mut open = start;

            for position in positions.clone() {
                match get_cell(self, line, position) {
                    Cell::Empty => {}
                    Cell::Fixed => {
                        open = next(position);
                    }
                    Cell::Movable if position == open => {
                        open = next(open);
                    }
                    Cell::Movable => {
                        set_cell(self, line, position, Cell::Empty);
                        set_cell(self, line, open, Cell::Movable);
                        open = next(open);
                    }
                }
            }
        }
    }

    fn total_load(&self) -> usize {
        self.cells
            .chunks_exact(self.width)
            .enumerate()
            .map(|(y, row)| {
                (self.height - y) * row.iter().filter(|cell| **cell == Cell::Movable).count()
            })
            .sum()
    }
}

impl Index<(usize, usize)> for Grid {
    type Output = Cell;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        unsafe { self.cells.get_unchecked(index.0 + index.1 * self.width) }
    }
}

impl IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        unsafe { self.cells.get_unchecked_mut(index.0 + index.1 * self.width) }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Cell {
    Empty,
    Fixed,
    Movable,
}

impl From<u8> for Cell {
    fn from(value: u8) -> Self {
        match value {
            b'.' => Self::Empty,
            b'#' => Self::Fixed,
            b'O' => Self::Movable,
            _ => unreachable!(),
        }
    }
}
