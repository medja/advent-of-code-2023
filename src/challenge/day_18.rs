pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(measure_area(
        input,
        parse_simple_direction,
        parse_simple_distance,
    ))
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(measure_area(
        input,
        parse_complex_direction,
        parse_complex_distance,
    ))
}

fn parse_simple_direction(input: &[u8]) -> u8 {
    input[0]
}

fn parse_complex_direction(input: &[u8]) -> u8 {
    input[input.len() - 2]
}

fn parse_simple_distance(input: &[u8]) -> i64 {
    let index = 2 + input[2..].iter().position(|char| *char == b' ').unwrap();

    input[2..index]
        .iter()
        .fold(0, |acc, char| acc * 10 + char - b'0') as i64
}

fn parse_complex_distance(input: &[u8]) -> i64 {
    input[input.len() - 7..input.len() - 2]
        .iter()
        .fold(0, |acc, &char| {
            let value = if char > b'9' {
                char - b'a' + 10
            } else {
                char - b'0'
            };

            (acc << 4) + (value as i64)
        })
}

fn measure_area(
    input: &[&str],
    parse_direction: fn(&[u8]) -> u8,
    parse_distance: fn(&[u8]) -> i64,
) -> i64 {
    let mut x = 0;
    let mut area = 0;

    let (mut up, mut left) = find_initial_state(input, parse_direction);

    for line in input.iter().map(|line| line.as_bytes()) {
        let distance = parse_distance(line);

        match parse_direction(line) {
            b'U' | b'3' => {
                if up {
                    area -= x * (distance + 1);
                } else {
                    area -= x * distance - (left as i64);
                }

                up = true;
            }
            b'D' | b'1' => {
                if up {
                    area += (x + 1) * distance + (!left as i64);
                } else {
                    area += (x + 1) * (distance + 1);
                }

                up = false;
            }
            b'L' | b'2' => {
                if up {
                    area += x;
                }

                x -= distance;

                if !up {
                    area -= x + 1;
                }

                left = true;
            }
            b'R' | b'0' => {
                if !up {
                    area -= x + 1;
                }

                x += distance;

                if up {
                    area += x;
                }

                left = false;
            }
            _ => unreachable!(),
        }
    }

    area
}

fn find_initial_state(input: &[&str], parse_direction: fn(&[u8]) -> u8) -> (bool, bool) {
    let up = input
        .iter()
        .rev()
        .find_map(|line| match parse_direction(line.as_bytes()) {
            b'U' | b'3' => Some(true),
            b'D' | b'1' => Some(false),
            _ => None,
        })
        .unwrap();

    let left = input
        .iter()
        .rev()
        .find_map(|line| match parse_direction(line.as_bytes()) {
            b'L' | b'2' => Some(true),
            b'R' | b'0' => Some(false),
            _ => None,
        })
        .unwrap();

    (up, left)
}
