use std::cmp::Ordering;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut rows = Vec::with_capacity(17);
    let mut columns = Vec::with_capacity(17);

    let result = input
        .split(|line| line.is_empty())
        .map(|image| {
            parse_image(image, &mut rows, &mut columns);

            match find_clean_reflection(&rows) {
                Some(index) => index * 100,
                None => find_clean_reflection(&columns).unwrap(),
            }
        })
        .sum::<usize>();

    Ok(result)
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut rows = Vec::with_capacity(17);
    let mut columns = Vec::with_capacity(17);

    let result = input
        .split(|line| line.is_empty())
        .map(|image| {
            parse_image(image, &mut rows, &mut columns);

            match find_dirty_reflection(&rows) {
                Some(index) => index * 100,
                None => find_dirty_reflection(&columns).unwrap(),
            }
        })
        .sum::<usize>();

    Ok(result)
}

fn parse_image(image: &[&str], rows: &mut Vec<u32>, columns: &mut Vec<u32>) {
    rows.clear();
    columns.clear();
    columns.resize(image[0].len(), 0);

    for row in image {
        let mut row_value = 0;

        for (i, value) in row.bytes().enumerate() {
            let bit = (value == b'#') as u32;
            row_value = row_value << 1 | bit;
            columns[i] = columns[i] << 1 | bit;
        }

        rows.push(row_value);
    }
}

fn find_clean_reflection(lines: &[u32]) -> Option<usize> {
    for (index, window) in lines.windows(2).enumerate() {
        if window[0] != window[1] {
            continue;
        }

        let before = &lines[..index];
        let after = &lines[index + 2..];

        if before
            .iter()
            .rev()
            .zip(after)
            .all(|(first, second)| first == second)
        {
            return Some(index + 1);
        }
    }

    None
}

fn find_dirty_reflection(lines: &[u32]) -> Option<usize> {
    for (index, window) in lines.windows(2).enumerate() {
        let dirty = match compare_lines(window[0], window[1]) {
            Some(dirty) => dirty,
            None => continue,
        };

        let before = &lines[..index];
        let after = &lines[index + 2..];

        let matches =
            before
                .iter()
                .rev()
                .zip(after)
                .try_fold(dirty, |was_dirty, (first, second)| {
                    match compare_lines(*first, *second) {
                        Some(true) if was_dirty => None,
                        Some(is_dirty) => Some(is_dirty | was_dirty),
                        None => None,
                    }
                });

        if matches!(matches, Some(true)) {
            return Some(index + 1);
        }
    }

    None
}

fn compare_lines(first: u32, second: u32) -> Option<bool> {
    match (first ^ second).count_ones().cmp(&1) {
        Ordering::Less => Some(false),
        Ordering::Equal => Some(true),
        Ordering::Greater => None,
    }
}
