pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(input
        .iter()
        .map(|line| predict_next(line.split_ascii_whitespace()))
        .sum::<i32>())
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(input
        .iter()
        .map(|line| predict_next(line.split_ascii_whitespace().rev()))
        .sum::<i32>())
}

fn predict_next<'a>(input: impl Iterator<Item = &'a str>) -> i32 {
    let mut values = input
        .map(|value| value.parse().unwrap())
        .collect::<Vec<i32>>();

    let mut result = *values.last().unwrap();

    while compute_deltas(&mut values) {
        result += *values.last().unwrap();
    }

    result
}

// returns true if at least one of the deltas is non-zero
fn compute_deltas(values: &mut Vec<i32>) -> bool {
    // keeps track of the bits set in any of the deltas
    let mut updated = 0;

    for i in 0..values.len() - 1 {
        let value = values[i + 1] - values[i];
        values[i] = value;
        updated |= value;
    }

    if updated == 0 {
        return false;
    }

    values.pop();
    true
}
