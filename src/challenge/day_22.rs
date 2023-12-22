use rustc_hash::FxHashSet;
use std::cmp::Ordering;

const GRID_SIZE: usize = 10;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let bricks = parse_bricks(input);
    // top down view of the grid
    let mut grid = [Point::default(); GRID_SIZE * GRID_SIZE];
    let mut unstable = vec![false; bricks.len()];

    for (id, brick) in bricks.into_iter().enumerate() {
        // z position at which the brick will land
        let mut max_z = 0;
        // id/index of the *single* brick supporting the current brick (or usize::MAX)
        let mut support_id = usize::MAX;

        for index in brick.indexes() {
            match grid[index].z.cmp(&max_z) {
                Ordering::Greater => {
                    max_z = grid[index].z;
                    support_id = grid[index].brick;
                }
                Ordering::Equal if grid[index].brick != support_id => {
                    support_id = usize::MAX;
                }
                _ => {}
            }
        }

        update_grid(id, brick, max_z, &mut grid);

        if support_id != usize::MAX {
            unstable[support_id] = true;
        }
    }

    Ok(unstable.iter().filter(|unstable| !**unstable).count())
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let bricks = parse_bricks(input);
    // top down view of the grid
    let mut grid = [Point::default(); GRID_SIZE * GRID_SIZE];
    // ids/indexes of bricks that cause each brick to fall
    let mut falls_when = Vec::<FxHashSet<usize>>::with_capacity(bricks.len());
    // ids/indexes of bricks supporting the current brick (only used inside the loop)
    let mut supports = FxHashSet::default();

    for (id, brick) in bricks.into_iter().enumerate() {
        // z position at which the brick will land
        let mut max_z = 0;

        for index in brick.indexes() {
            if grid[index].z > max_z {
                max_z = grid[index].z;
                supports.clear();
            }

            if max_z != 0 && max_z == grid[index].z {
                supports.insert(grid[index].brick);
            }
        }

        update_grid(id, brick, max_z, &mut grid);

        if max_z == 0 {
            // supports might contain stale ids from the previous iteration
            supports.clear();
        }

        falls_when.push(find_load_bearing_bricks(&supports, &falls_when));
    }

    Ok(falls_when.into_iter().map(|x| x.len()).sum::<usize>())
}

fn update_grid(id: usize, brick: Brick, max_z: usize, grid: &mut [Point; GRID_SIZE * GRID_SIZE]) {
    let height = brick.z1 - brick.z0 + 1;
    let next_z = max_z + height;

    for index in brick.indexes() {
        grid[index] = Point {
            brick: id,
            z: next_z,
        };
    }
}

// A brick will fall when removing a brick that causes all of its supporting bricks to fall as well
fn find_load_bearing_bricks(
    supports: &FxHashSet<usize>,
    falls_when: &[FxHashSet<usize>],
) -> FxHashSet<usize> {
    if supports.is_empty() {
        return FxHashSet::default();
    }

    let single = supports.len() == 1;
    let mut supports = supports.iter();
    let first_id = *supports.next().unwrap();
    let mut builder = falls_when[first_id].clone();

    // removing the single supporting brick will cause this brick to fall as well
    if single {
        builder.insert(first_id);
        return builder;
    }

    // compute intersection of falls_when sets of all supporting bricks
    for id in supports {
        let set = &falls_when[*id];
        builder.retain(|x| set.contains(x));
    }

    builder
}

fn parse_bricks(input: &[&str]) -> Vec<Brick> {
    let mut bricks = input
        .iter()
        .map(|line| Brick::new(line.as_bytes()))
        .collect::<Vec<_>>();

    bricks.sort_unstable_by_key(|brick| brick.z0);
    bricks
}

struct Brick {
    // assume *0 will always be less than or equal to *1
    x0: usize,
    y0: usize,
    z0: usize,
    x1: usize,
    y1: usize,
    z1: usize,
}

impl Brick {
    fn new(input: &[u8]) -> Self {
        let mut iter = input.split(|char| !char.is_ascii_digit()).map(parse_digit);

        Self {
            x0: iter.next().unwrap(),
            y0: iter.next().unwrap(),
            z0: iter.next().unwrap(),
            x1: iter.next().unwrap(),
            y1: iter.next().unwrap(),
            z1: iter.next().unwrap(),
        }
    }

    fn indexes(&self) -> impl Iterator<Item = usize> + '_ {
        // defined as 2 iterators so it gets inlined as 2 loops which reduces branching
        (self.y0..=self.y1).flat_map(|y| {
            (0..self.x1 - self.x0 + 1).scan(self.x0 + y * GRID_SIZE, |index, _| {
                let current = *index;
                // incrementing a single value is faster than recomputing the index from x and y
                *index += 1;
                Some(current)
            })
        })
    }
}

#[derive(Copy, Clone)]
struct Point {
    brick: usize,
    z: usize,
}

impl Default for Point {
    fn default() -> Self {
        Self {
            brick: usize::MAX,
            z: 0,
        }
    }
}

fn parse_digit(value: &[u8]) -> usize {
    value
        .iter()
        .fold(0, |acc, char| acc * 10 + (char - b'0') as usize)
}
