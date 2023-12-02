const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(input
        .iter()
        .filter_map(|game| find_possible_game_id(game))
        .sum::<u32>())
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(input
        .iter()
        .map(|game| find_game_cube_power(game))
        .sum::<u32>())
}

fn find_possible_game_id(game: &str) -> Option<u32> {
    let mut parts = game.split_ascii_whitespace();
    parts.next(); // skip "Game"
    let id = parts.next().unwrap();

    while let (Some(count), Some(color)) = (parts.next(), parts.next()) {
        let count: u32 = count.parse().unwrap();

        let impossible = match color.as_bytes()[0] {
            b'r' => count > MAX_RED,
            b'g' => count > MAX_GREEN,
            b'b' => count > MAX_BLUE,
            _ => unreachable!(),
        };

        if impossible {
            return None;
        }
    }

    // Parse game id without the trailing ":"
    Some(id[..(id.len() - 1)].parse().unwrap())
}

fn find_game_cube_power(game: &str) -> u32 {
    let mut min_red = 0;
    let mut min_green = 0;
    let mut min_blue = 0;

    let mut parts = game.split_ascii_whitespace();
    parts.next(); // skip "Game"
    parts.next(); // skip game id

    while let (Some(count), Some(color)) = (parts.next(), parts.next()) {
        let count: u32 = count.parse().unwrap();

        match color.as_bytes()[0] {
            b'r' => min_red = min_red.max(count),
            b'g' => min_green = min_green.max(count),
            b'b' => min_blue = min_blue.max(count),
            _ => unreachable!(),
        }
    }

    min_red * min_green * min_blue
}
