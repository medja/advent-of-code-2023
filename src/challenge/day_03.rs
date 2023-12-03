use std::ops::RangeInclusive;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(sum_part_numbers(input))
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(sum_gear_ratios(input))
}

fn sum_part_numbers(input: &[&str]) -> u32 {
    let line_len = input[0].len();

    let mut number = 0u32;
    let mut is_part_number = false;
    let mut part_number_sum = 0u32;

    for (y, line) in input.iter().enumerate() {
        for (x, char) in line.bytes().enumerate() {
            if char.is_ascii_digit() {
                number = number * 10 + (char - b'0') as u32;

                if !is_part_number {
                    is_part_number = contains_part_number(x, y, input, line_len);
                }
            } else if number > 0 {
                // Assumes the number can't be 0
                if is_part_number {
                    part_number_sum += number;
                }

                number = 0;
                is_part_number = false;
            }
        }
    }

    if number > 0 && is_part_number {
        part_number_sum += number;
    }

    part_number_sum
}

fn contains_part_number(x: usize, y: usize, input: &[&str], line_len: usize) -> bool {
    input[find_neighbors(y, input.len())].iter().any(|line| {
        line.as_bytes()[find_neighbors(x, line_len)]
            .iter()
            .any(|&char| !char.is_ascii_digit() && char != b'.')
    })
}

fn sum_gear_ratios(input: &[&str]) -> u32 {
    let line_len = input[0].len();

    input
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.bytes()
                .enumerate()
                .filter(|(_, char)| *char == b'*')
                .filter_map(move |(x, _)| find_gear_ratio(x, y, input, line_len))
        })
        .sum()
}

fn find_gear_ratio(x: usize, y: usize, input: &[&str], line_len: usize) -> Option<u32> {
    let mut iter = input[find_neighbors(y, input.len())]
        .iter()
        .flat_map(|line| {
            let line = line.as_bytes();

            let range = if line[x].is_ascii_digit() {
                x..=x
            } else {
                find_neighbors(x, line_len)
            };

            range.filter_map(|x| find_number(x, line))
        });

    // A gear must be adjacent to exactly two part numbers
    if let (Some(first), Some(second), None) = (iter.next(), iter.next(), iter.next()) {
        Some(first * second)
    } else {
        None
    }
}

fn find_number(index: usize, line: &[u8]) -> Option<u32> {
    if !line[index].is_ascii_digit() {
        return None;
    }

    // Find the begining of the number
    let offset = line[..index]
        .iter()
        .rev()
        .take_while(|char| char.is_ascii_digit())
        .count();

    let number = line[index - offset..]
        .iter()
        .take_while(|char| char.is_ascii_digit())
        .fold(0u32, |sum, char| sum * 10 + (char - b'0') as u32);

    Some(number)
}

fn find_neighbors(index: usize, limit: usize) -> RangeInclusive<usize> {
    let next = index + 1;

    // Assumes the input has more than 1 line and more than 1 column
    if index == 0 {
        next..=next
    } else if next == limit {
        index - 1..=index
    } else {
        index - 1..=next
    }
}
