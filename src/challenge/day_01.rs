pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(solve(input, parse_simple))
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(solve(input, parse_complex))
}

fn solve(input: &[&str], parse_digits: fn(bytes: &[u8]) -> (u8, u8)) -> u32 {
    input
        .iter()
        .map(|line| {
            let (first, last) = parse_digits(line.as_bytes());
            (first as u32) * 10 + (last as u32)
        })
        .sum()
}

fn parse_simple(bytes: &[u8]) -> (u8, u8) {
    let first = bytes.iter().find(|char| char.is_ascii_digit());
    let last = bytes.iter().rev().find(|char| char.is_ascii_digit());

    (
        first.map(|char| char - b'0').unwrap(),
        last.map(|char| char - b'0').unwrap(),
    )
}

fn parse_complex(bytes: &[u8]) -> (u8, u8) {
    let first = (0..bytes.len())
        .find_map(|i| parse_complex_digit(&bytes[i..]))
        .unwrap();

    let last = (0..bytes.len())
        .rev()
        .find_map(|i| parse_complex_digit(&bytes[i..]))
        .unwrap();

    (first, last)
}

fn parse_complex_digit(bytes: &[u8]) -> Option<u8> {
    match bytes {
        [byte @ b'0'..=b'9', ..] => Some(byte - b'0'),
        [b'o', b'n', b'e', ..] => Some(1),
        [b't', b'w', b'o', ..] => Some(2),
        [b't', b'h', b'r', b'e', b'e', ..] => Some(3),
        [b'f', b'o', b'u', b'r', ..] => Some(4),
        [b'f', b'i', b'v', b'e', ..] => Some(5),
        [b's', b'i', b'x', ..] => Some(6),
        [b's', b'e', b'v', b'e', b'n', ..] => Some(7),
        [b'e', b'i', b'g', b'h', b't', ..] => Some(8),
        [b'n', b'i', b'n', b'e', ..] => Some(9),
        _ => None,
    }
}
