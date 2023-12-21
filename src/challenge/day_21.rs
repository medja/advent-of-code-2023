use rustc_hash::FxHashSet;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(simulate(input, 64).len())
}

// Once we've reached the edge of the 2rd grid (counting from 0) a pattern starts to emerge:
//  - - - - -
// |*|G|K|H|*|
//  - - - - -
// |G|C|B|D|H|
//  - - - - -
// |N|B|A|B|L|
//  - - - - -
// |J|F|B|E|I|
//  - - - - -
// |*|J|M|I|*|
//  - - - - -
// A, B, C, ... are arrangements of the potential steps
//
// Our grid has a size of 131.
// We reach the edge of it for the first time (due to starting at the center) at 65 steps.
// 26501365 steps gets us exactly to the edge of the (26501365 - 65) / 131 = 202300th grid.
//
// The A and B grids alternate between A and B arrangements after every cycle.
// These grids are also the ones that have been fully "discovered".
// At the cycle where we reach the edge of the Nth grid (counting from 0!) there are
// (N - 1)^2 A grids and N^2 B grids.
//
// The C, D, E, and F grids make up the largest part of the edge.
// Their exact arrangements repeat along the edges of the diamond every time we reach the edge
// of the next grid.
// Once we reach the Nth grid, each of them repeats N - 1 times.
//
// The G, H, I, and J grids make up the smaller part of the edge.
// They repeat similatly to the C, D, E, and F grids. But they show up more times.
// Once we reach the Nth grid, each of them repeats N times.
//
// Finally the K, L, M and N grids repeat exactly as well.
// Each of them only shows up once.
pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    const STEPS: u64 = 26501365;

    // assume the grid is square
    let size = input.len() as isize;

    // size / 2 gets us to the edge of the first grid, each additinal size steps gets us
    // to the edge of the next grid.
    // This is just enough to observe all of the unique grid arrangements.
    let min_steps = 2 * size + size / 2;
    let positions = simulate(input, min_steps as usize);

    let mut center_a = 0u64; // A grids
    let mut center_b = 0u64; // B grids
    let mut big_edge = 0u64; // C, D, E, and F grids
    let mut small_edge = 0u64; // G, H, I, and J grids
    let mut corners = 0u64; // K, L, M and N grids

    for (x, y) in positions {
        match (grid_index(x, size), grid_index(y, size)) {
            (0, 0) => center_a += 1,
            (0, 1) => center_b += 1,
            (-1, -1) | (1, -1) | (-1, 1) | (1, 1) => big_edge += 1,
            (-1, -2) | (1, -2) | (-1, 2) | (1, 2) => small_edge += 1,
            (0, -2) | (0, 2) | (-2, 0) | (2, 0) => corners += 1,
            _ => {}
        }
    }

    // The number (starting from 0) of the grid we've reached horizontally or vertically
    let count = STEPS / size as u64;

    Ok(center_a * (count - 1).pow(2)
        + center_b * count.pow(2)
        + big_edge * (count - 1)
        + small_edge * count
        + corners)
}

fn simulate(input: &[&str], steps: usize) -> FxHashSet<(isize, isize)> {
    // assume the grid is square
    let size = input.len() as isize;
    let start = find_start(input);

    let mut current_steps = FxHashSet::default();
    let mut next_steps = FxHashSet::default();
    current_steps.insert(start);

    for _ in 0..steps {
        for (x, y) in current_steps.drain() {
            for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                let x = x + dx;
                let y = y + dy;

                let gx = x.rem_euclid(size) as usize;
                let gy = y.rem_euclid(size) as usize;

                if input[gy].as_bytes()[gx] == b'#' {
                    continue;
                }

                next_steps.insert((x, y));
            }
        }

        (current_steps, next_steps) = (next_steps, current_steps);
    }

    current_steps
}

fn find_start(input: &[&str]) -> (isize, isize) {
    input
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.bytes()
                .position(|char| char == b'S')
                .map(|x| (x as isize, y as isize))
        })
        .unwrap()
}

fn grid_index(position: isize, size: isize) -> isize {
    if position < 0 {
        (position + 1) / size - 1
    } else {
        position / size
    }
}
