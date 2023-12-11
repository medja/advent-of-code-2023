pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(Analysis::build(input, 1).sum_distances())
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(Analysis::build(input, 999999).sum_distances())
}

struct Analysis {
    // coordinates of each galaxy, sorted in ascending order and adjusted for expansion
    x_coordinates: Vec<usize>,
    y_coordinates: Vec<usize>,
}

impl Analysis {
    fn build(image: &[&str], multiplier: usize) -> Self {
        let mut y_coordinates = Vec::new();
        let mut x_coordinates_count = vec![0; image[0].len()];
        let mut y_expansion = 0;

        // Scan the image and keep track of y coordinates of every galaxy (in ascending order)
        // Use this as an opportunity to count how many times we've seen a galaxy at every x coordinate
        for (y, row) in image.iter().enumerate() {
            let mut empty = true;

            for (x, pixel) in row.bytes().enumerate() {
                if pixel != b'#' {
                    continue;
                }

                y_coordinates.push(y + y_expansion);
                x_coordinates_count[x] += 1;
                empty = false;
            }

            // Every empty row gets expanded into multiple rows
            if empty {
                y_expansion += multiplier;
            }
        }

        let mut x_coordinates = Vec::with_capacity(y_coordinates.len());
        let mut x_expansion = 0;

        // Use the x coordinate counts to generate a vector of their coordinates
        for (x, count) in x_coordinates_count.into_iter().enumerate() {
            // Every empty column gets expanded into multiple columns
            if count == 0 {
                x_expansion += multiplier;
                continue;
            }

            for _ in 0..count {
                x_coordinates.push(x + x_expansion);
            }
        }

        Self {
            x_coordinates,
            y_coordinates,
        }
    }

    fn sum_distances(&self) -> u64 {
        (sum_distances(&self.x_coordinates) + sum_distances(&self.y_coordinates)) as u64
    }
}

// # Solved using a hint to write down my previous solution as an equation
//
// The sum of the minimum distances between all pairs of galaxies can be computed by adding them together
// the differences in the x and y coordinates of all of the galaxies (assuming they're adjusted for expansion).
// So the addition of differences of x and y coordinates. This means the x and y components of the sum
// can be computed independently.
//
// Assuming there are 5 galaxies and the coordinates are sorted in ascending order,
// the sum of the differences in x coordinates can be written down as:
// (x1 - x0) + (x2 - x0) + (x3 - x0) + (x4 - x0) + (x2 - x1) + (x3 - x1) + (x4 - x1) + (x3 - x2) + (x4 - x2) + (x4 - x3)
// The parentheses aren't necessary, so this can be simplified to:
// x1 - x0 + x2 - x0 + x3 - x0 + x4 - x0 + x2 - x1 + x3 - x1 + x4 - x1 + x3 - x2 + x4 - x2 + x4 - x3
// By combining the terms we get:
// (-4)*x0 + (-2)*x1 + (0)*x2 + (+2)*x3 + (+4)*x4
//
// The number of times each term is added is equal to `N - 2i - 1`
// Here `N` is the number of galaxies and `i` is the index of the galaxy (starting at 0).
// Repeating this process for y coordinates and adding the results yield the sum of all of the distances.
fn sum_distances(coordinates: &[usize]) -> i64 {
    let offset = coordinates.len() as i64 - 1;

    coordinates
        .iter()
        .enumerate()
        // computes `x_i * (N - 2*i - 1)` for each coordinate (`x_i`)
        .map(|(i, value)| (*value as i64) * ((i as i64) * 2 + offset))
        .sum::<i64>()
}
