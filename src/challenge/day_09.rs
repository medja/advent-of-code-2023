pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(input.iter().map(|line| predict_next(line)).sum::<i32>())
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(input.iter().map(|line| predict_previous(line)).sum::<i32>())
}

fn parse_values(input: &str) -> Vec<i32> {
    input
        .split_ascii_whitespace()
        .map(|value| value.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
}

fn predict_next(input: &str) -> i32 {
    let mut values = parse_values(input);
    let mut result = *values.last().unwrap();

    while compute_deltas(&mut values) {
        result += *values.last().unwrap();
    }

    result
}

fn predict_previous(input: &str) -> i32 {
    let mut values = parse_values(input);
    let mut result = values[0];
    let mut subtract = true;

    while compute_deltas(&mut values) {
        // a - (b - (c - (d - e))) =
        // a -  b +  c -  d + e
        if subtract {
            result -= values[0];
        } else {
            result += values[0];
        }

        subtract = !subtract;
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
