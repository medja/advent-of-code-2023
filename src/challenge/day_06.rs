use std::cmp::Ordering;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let limits = parse_numbers(&input[0][9..]);
    let records = parse_numbers(&input[1][9..]);

    Ok(limits
        .zip(records)
        .map(|(limit, record)| compute_error_margin(solve_min_time(limit, record), limit))
        .product::<u64>())
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let limit = parse_number(&input[0][9..]);
    let record = parse_number(&input[1][9..]);
    Ok(compute_error_margin(find_min_time(limit, record), limit))
}

fn parse_numbers(input: &str) -> impl Iterator<Item = u64> + '_ {
    input
        .split_ascii_whitespace()
        .map(|number| number.parse().unwrap())
}

fn parse_number(input: &str) -> u64 {
    input
        .split_ascii_whitespace()
        .flat_map(|number| number.bytes())
        .fold(0u64, |acc, digit| (acc * 10) + (digit - b'0') as u64)
}

fn compute_error_margin(min_time: u64, limit: u64) -> u64 {
    // same as `max_time - min_time + 1` (see solve_min_time)
    limit - 2 * min_time + 1
}

// Find the minimum amount of time the button has to be pressed to beat the record
// This uses binary search to find the solution, see find_min_time for a math based solution
fn find_min_time(limit: u64, record: u64) -> u64 {
    let mut left = 0;
    let mut right = limit;

    while left < right {
        let mid = (left + right) / 2;
        let distance = mid * (limit - mid);

        match distance.cmp(&record) {
            Ordering::Less => left = mid + 1,
            Ordering::Equal => return mid + 1,
            Ordering::Greater => right = mid,
        }
    }

    left
}

// Math based alternative to find_min_time
/*
 * time * (limit - time) = record
 * time * limit - time^2 = record
 * -time^2 + time * limit - record = 0
 *
 * // quadratic formula: x = (-b +_ sqrt(b^2 - 4ac)) / 2a
 * (-limit +_ sqrt(limit^2 - 4 * record)) / -2
 * (limit +_ sqrt(limit^2 - 4 * record)) / 2
 *
 * time_min = (limit - sqrt(limit^2 - 4 * record)) / 2
 * time_max = (limit + sqrt(limit^2 - 4 * record)) / 2
 *
 * // next_whole produces the first whole number bigger than the input
 * // previous_whole produce the first whole number smaller than the input
 * result = next_whole(time_max) - previous_whole(time_min) + 1
 *
 * // if `f(x)` is `x * (limit - x)`
 * // notice that `f(limit - x)` actually expands to:
 * // `(limit - x) * (limit - limit + x) = (limit - x) * x`
 * // this means that next_whole(time_max) == limit - previous_whole(time_min)
 * result = limit - 2 * previous_whole(time_min) + 1
 */
fn solve_min_time(limit: u64, record: u64) -> u64 {
    let min = ((limit as f64) - ((limit * limit - 4 * record) as f64).sqrt()) / 2f64;
    let min_ceil = min.ceil();

    if min == min_ceil {
        min as u64 - 1
    } else {
        min_ceil as u64
    }
}
