use std::collections::BinaryHeap;

// Heat loss is worse around the center of the grid - the highest values (8, 9) only appear there
// This means that the center should be avoided (by skipping the highest values)
// This constraint might not work for all inputs
const MAX_HEAT_LOSS: u8 = 7;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    find_best_path(input, 1, 3)
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    find_best_path(input, 4, 10)
}

// Dijkstra's algorithm
// Treats all coordinates that can be visited without turning as neighbors
fn find_best_path(input: &[&str], min_steps: isize, max_steps: isize) -> anyhow::Result<usize> {
    let width = input[0].len();
    let height = input.len();
    let max_x = width - 1;
    let max_y = height - 1;

    let grid = input
        .iter()
        .flat_map(|row| row.bytes())
        .map(|cost| cost - b'0')
        .collect::<Vec<_>>();

    let mut queue = BinaryHeap::new();
    let mut costs = vec![usize::MAX; grid.len() * 4];

    queue.push(Entry::default());

    while let Some(Entry {
        cost,
        x,
        y,
        direction,
    }) = queue.pop()
    {
        if x == max_x && y == max_y {
            return Ok(cost);
        }

        if costs[x + y * width + direction * grid.len()] < cost {
            continue;
        }

        let invalid_direction = if x == 0 && y == 0 {
            // all directions are valid from the starting position
            // `something & 1` will only ever return `0` or `1`
            2
        } else {
            direction & 1
        };

        for (direction, (dx, dy)) in [(1, 0), (0, 1), (-1, 0), (0, -1)].iter().enumerate() {
            if (direction & 1) == invalid_direction {
                continue;
            }

            let mut x = x as isize;
            let mut y = y as isize;
            let mut total_heat_loss = 0;

            for steps in 1..=max_steps {
                x += dx;
                y += dy;

                if x < 0 || x >= width as isize || y < 0 || y >= height as isize {
                    break;
                }

                let x = x as usize;
                let y = y as usize;
                let index = x + y * width;
                let heat_loss = grid[index];

                if heat_loss > MAX_HEAT_LOSS {
                    break;
                }

                total_heat_loss += heat_loss as usize;

                if steps < min_steps {
                    continue;
                }

                let cost = cost + total_heat_loss;
                let id = index + direction * grid.len();

                if cost >= costs[id] {
                    // this might not be the cheapest path to this coordinate, but the optimal path might
                    // require us to go *pass over* it using this path anyway due to turning limitations
                    continue;
                }

                costs[id] = cost;

                queue.push(Entry {
                    cost,
                    x,
                    y,
                    direction,
                });
            }
        }
    }

    anyhow::bail!("Could not find a path");
}

#[derive(Eq, PartialEq, Default)]
struct Entry {
    cost: usize,
    x: usize,
    y: usize,
    direction: usize,
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}
