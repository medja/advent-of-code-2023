pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let analysis = Analysis::build(input);
    Ok(analysis.sum_distances(1))
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let analysis = Analysis::build(input);
    Ok(analysis.sum_distances(999999))
}

struct Analysis {
    galaxies: Vec<(usize, usize)>,
    empty_space_x: Vec<usize>,
    empty_space_y: Vec<usize>,
}

impl Analysis {
    fn build(image: &[&str]) -> Self {
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
            empty_space_x: collect_mask_indices(empty_space_x),
            empty_space_y: collect_mask_indices(empty_space_y),
        }
    }

    fn sum_distances(&self, multiplier: usize) -> u64 {
        let mut sum = 0;

        for start in 0..self.galaxies.len() - 1 {
            for end in start + 1..self.galaxies.len() {
                sum += self.find_distance(start, end, multiplier) as u64;
            }
        }

        sum
    }

    fn find_distance(&self, start: usize, end: usize, multiplier: usize) -> usize {
        let start = self.galaxies[start];
        let end = self.galaxies[end];

        let min_x = start.0.min(end.0);
        let max_x = start.0.max(end.0);
        let min_y = start.1.min(end.1);
        let max_y = start.1.max(end.1);

        let expanded_x = count_items_between(min_x, max_x, &self.empty_space_x);
        let expanded_y = count_items_between(min_y, max_y, &self.empty_space_y);

        (max_x - min_x) + (max_y - min_y) + (expanded_x + expanded_y) * multiplier
    }
}

fn collect_mask_indices(mask: Vec<bool>) -> Vec<usize> {
    mask.into_iter()
        .enumerate()
        .filter(|(_, value)| *value)
        .map(|(index, _)| index)
        .collect()
}

fn count_items_between<T: PartialOrd>(min: T, max: T, values: &[T]) -> usize {
    values
        .iter()
        .skip_while(|value| **value < min)
        .take_while(|value| **value < max)
        .count()
}
