pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(Analysis::build(input, 1).sum_distances())
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(Analysis::build(input, 999999).sum_distances())
}

struct Analysis {
    galaxies: Vec<(usize, usize)>,
    empty_space_x: Vec<usize>,
    empty_space_y: Vec<usize>,
}

impl Analysis {
    fn build(image: &[&str], multiplier: usize) -> Self {
        let mut galaxies = Vec::new();
        let mut empty_space_x = vec![true; image[0].len()];
        let mut empty_space_y = vec![true; image.len()];

        for (y, row) in image.iter().enumerate() {
            for (x, pixel) in row.bytes().enumerate() {
                if pixel != b'#' {
                    continue;
                }

                galaxies.push((x, y));
                empty_space_x[x] = false;
                empty_space_y[y] = false;
            }
        }

        Self {
            galaxies,
            empty_space_x: accumulate_empty_spaces(empty_space_x, multiplier),
            empty_space_y: accumulate_empty_spaces(empty_space_y, multiplier),
        }
    }

    fn sum_distances(&self) -> u64 {
        let mut sum = 0;

        for (i, &start) in self.galaxies.iter().enumerate() {
            for &end in &self.galaxies[i + 1..] {
                sum += self.find_distance(start, end) as u64;
            }
        }

        sum
    }

    fn find_distance(&self, start: (usize, usize), end: (usize, usize)) -> usize {
        let min_x = start.0.min(end.0);
        let max_x = start.0.max(end.0);
        let min_y = start.1.min(end.1);
        let max_y = start.1.max(end.1);

        // This function is very hot and the extra boundry checks from .get when accessing empty_space_x
        // and empty_space_y take up a significant amount of time (~10% on the total time my machine).
        // Safety: empty_space_x contains values for each column of the image and empty_space_y contains
        // values for each row of the image. The x and y values are coordinates of the same image.
        // So this access never goes out of bounds.
        let expanded_x = unsafe {
            self.empty_space_x.get_unchecked(max_x) - self.empty_space_x.get_unchecked(min_x)
        };

        let expanded_y = unsafe {
            self.empty_space_y.get_unchecked(max_y) - self.empty_space_y.get_unchecked(min_y)
        };

        (max_x - min_x) + (max_y - min_y) + expanded_x + expanded_y
    }
}

fn accumulate_empty_spaces(mask: Vec<bool>, multiplier: usize) -> Vec<usize> {
    mask.into_iter()
        .scan(0, |count, value| {
            if value {
                *count += multiplier;
            }

            Some(*count)
        })
        .collect()
}
