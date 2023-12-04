pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let offsets = (
        input[0].bytes().position(|char| char == b':').unwrap(),
        input[0].bytes().position(|char| char == b'|').unwrap(),
    );

    Ok(input
        .iter()
        .map(|card| score_card::<false>(card, offsets))
        .sum::<usize>())
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let offsets = (
        input[0].bytes().position(|char| char == b':').unwrap(),
        input[0].bytes().position(|char| char == b'|').unwrap(),
    );

    let mut card_counts = vec![1usize; input.len()];

    for (index, card) in input.iter().enumerate() {
        let score = score_card::<true>(card, offsets);
        let count = card_counts[index];

        for i in index + 1..(index + score + 1).min(card_counts.len()) {
            card_counts[i] += count;
        }
    }

    Ok(card_counts.into_iter().sum::<usize>())
}

fn score_card<const SUM: bool>(card: &str, offsets: (usize, usize)) -> usize {
    let mut score = 0;
    let mut winning_numbers = [0u8; 10];

    for (number, winning_number) in card[offsets.0 + 2..offsets.1 - 1]
        .split_ascii_whitespace()
        .zip(&mut winning_numbers)
    {
        *winning_number = number.parse().unwrap();
    }

    for number in card[offsets.1 + 2..].split_ascii_whitespace() {
        if !winning_numbers.contains(&number.parse().unwrap()) {
            continue;
        }

        if SUM {
            score += 1;
        } else if score == 0 {
            score = 1;
        } else {
            score *= 2;
        }
    }

    score
}
