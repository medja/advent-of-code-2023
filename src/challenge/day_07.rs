// Each card is represented by a single base-13 number.
// Because each hand consists of 5 cards, we can distinguish different strengths
// by shifting the the value of the hand by 5 * hand strength.
// This means that better hands always have a higher value weaker hands.
// Example representation (in base-13 with X, Y, and Z as placeholders):
// 5 of a kind: XXXXX 00000 00000 00000 00000 00000 00000
// full house:  00000 00000 XXYYX 00000 00000 00000 00000
// two pairs:   00000 00000 00000 00000 00000 XXYYZ 00000
const HAND_STRENGTH_OFFSET: u32 = 13u32.pow(5);
const FIVE_OF_A_KIND_OFFSET: u32 = 6 * HAND_STRENGTH_OFFSET;
const FOUR_OF_A_KIND_OFFSET: u32 = 5 * HAND_STRENGTH_OFFSET;
const FULL_HOUSE_OFFSET: u32 = 4 * HAND_STRENGTH_OFFSET;
const THREE_OF_A_KIND_OFFSET: u32 = 3 * HAND_STRENGTH_OFFSET;
const TWO_PAIRS_OFFSET: u32 = 2 * HAND_STRENGTH_OFFSET;
const ONE_PAIR_OFFSET: u32 = HAND_STRENGTH_OFFSET;
const HIGH_NUMBER_OFFSET: u32 = 0;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(solve::<false>(input))
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(solve::<true>(input))
}

struct Hand {
    score: u32,
    bid: u32,
}

fn solve<const JOKER: bool>(input: &[&str]) -> u32 {
    let mut hands = input
        .iter()
        .map(|line| {
            let score = parse_hand::<JOKER>(&line[..5]);
            let bid = line[6..].parse().unwrap();
            Hand { score, bid }
        })
        .collect::<Vec<_>>();

    hands.sort_unstable_by_key(|hand| hand.score);

    hands
        .into_iter()
        .enumerate()
        .map(|(rank, hand)| hand.bid * (rank as u32 + 1))
        .sum()
}

fn parse_hand<const JOKER: bool>(input: &str) -> u32 {
    let mut hand = 0;
    let mut card_counts = [0u8; 13];
    let mut joker_count: u8 = 0;

    for card in input.bytes().map(parse_card::<JOKER>) {
        hand = hand * 13 + card;

        if JOKER && card == 0 {
            joker_count += 1;
        } else {
            card_counts[card as usize] += 1;
        }
    }

    hand + find_hand_strength_offset(card_counts, joker_count)
}

fn parse_card<const JOKER: bool>(input: u8) -> u32 {
    if JOKER {
        parse_card_with_joker(input)
    } else {
        parse_card_without_joker(input)
    }
}

fn find_hand_strength_offset(card_counts: [u8; 13], joker_count: u8) -> u32 {
    // largest and second largest set of cards
    // (5, 0) for five of a kind, (3, 2) for full house, etc.
    let mut max = (0u8, 0u8);

    for count in card_counts {
        if count > max.0 {
            max.1 = max.0;
            max.0 = count;
        } else if count > max.1 {
            max.1 = count;
        }
    }

    max.0 += joker_count;

    match max {
        (5, _) => FIVE_OF_A_KIND_OFFSET,
        (4, _) => FOUR_OF_A_KIND_OFFSET,
        (3, 2) => FULL_HOUSE_OFFSET,
        (3, _) => THREE_OF_A_KIND_OFFSET,
        (2, 2) => TWO_PAIRS_OFFSET,
        (2, _) => ONE_PAIR_OFFSET,
        _ => HIGH_NUMBER_OFFSET,
    }
}

fn parse_card_without_joker(input: u8) -> u32 {
    match input {
        b'A' => 12u32,
        b'K' => 11u32,
        b'Q' => 10u32,
        b'J' => 9u32,
        b'T' => 8u32,
        b'9' => 7u32,
        b'8' => 6u32,
        b'7' => 5u32,
        b'6' => 4u32,
        b'5' => 3u32,
        b'4' => 2u32,
        b'3' => 1u32,
        b'2' => 0u32,
        _ => unreachable!(),
    }
}

fn parse_card_with_joker(input: u8) -> u32 {
    match input {
        b'A' => 12u32,
        b'K' => 11u32,
        b'Q' => 10u32,
        b'T' => 9u32,
        b'9' => 8u32,
        b'8' => 7u32,
        b'7' => 6u32,
        b'6' => 5u32,
        b'5' => 4u32,
        b'4' => 3u32,
        b'3' => 2u32,
        b'2' => 1u32,
        b'J' => 0u32,
        _ => unreachable!(),
    }
}
