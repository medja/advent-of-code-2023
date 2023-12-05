pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut values = input[0][7..]
        .split_ascii_whitespace()
        .map(|value| value.parse().unwrap())
        .collect::<Vec<u32>>();

    let mut lookups = Vec::<Rule>::new();

    for group in input[2..].split(|line| line.is_empty()) {
        lookups.clear();
        lookups.reserve(group.len() - 1);
        lookups.extend(group[1..].iter().map(|rule| parse_rule(rule)));

        for value in &mut values {
            let rule = lookups
                .iter()
                .find(|map| map.src_start <= *value && *value <= map.src_start + map.end_offset);

            if let Some(rule) = rule {
                if rule.dst_start < rule.src_start {
                    *value -= rule.src_start - rule.dst_start;
                } else {
                    *value += rule.dst_start - rule.src_start;
                }
            }
        }
    }

    Ok(*values.iter().min().unwrap())
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut current_ranges = parse_seed_ranges(&input[0][7..]);
    let mut next_ranges = Vec::<SeedRange>::new();
    let mut lookups = Vec::<Rule>::new();

    for group in input[2..].split(|line| line.is_empty()) {
        lookups.clear();
        lookups.reserve(group.len() - 1);
        lookups.extend(group[1..].iter().map(|rule| parse_rule(rule)));
        lookups.sort_by_key(|map| map.src_start);

        // Keep track of seeds as ranges, mapping (and splitting) them based on the rules
        for mut range in current_ranges.iter().cloned() {
            for rule in &lookups {
                let end_offset = if rule.src_start <= range.start
                    && range.start <= rule.src_start + rule.end_offset
                {
                    // The start of the current seed range intersects with the mapping rule
                    let start_offset = range.start - rule.src_start;
                    let start = rule.dst_start + start_offset;
                    let end_offset = (range.end - range.start).min(rule.end_offset - start_offset);
                    next_ranges.push(SeedRange::new(start, start + end_offset));

                    end_offset
                } else if range.start <= rule.src_start && rule.src_start <= range.end {
                    // The start of the mapping rule intersects the current seed range
                    let start = rule.dst_start;
                    let end_offset = rule.end_offset.min(range.end - range.start);
                    next_ranges.push(SeedRange::new(start, start + end_offset));

                    end_offset
                } else if range.end < rule.src_start {
                    // We've compared against all potential mapping rules
                    break;
                } else {
                    // The rule doesn't intersect the current seed range
                    continue;
                };

                // Advance the current range by the number of mapped seeds
                if range.start + end_offset == range.end {
                    // The max seed number is u32::MAX, so range.start + mapped + 1 might overflow
                    range = SeedRange::EMPTY;
                    break;
                } else {
                    range.start += end_offset + 1;
                }
            }

            // Add back the remaining (unmapped) seeds
            if range.is_non_empty() {
                next_ranges.push(range);
            }
        }

        std::mem::swap(&mut current_ranges, &mut next_ranges);
        next_ranges.clear();
    }

    Ok(current_ranges
        .iter()
        .map(|range| range.start)
        .min()
        .unwrap())
}

fn parse_rule(input: &str) -> Rule {
    let mut iter = input.split(' ');
    let dst_start = iter.next().unwrap().parse().unwrap();
    let src_start = iter.next().unwrap().parse().unwrap();
    let end_offset = iter.next().unwrap().parse().unwrap();
    Rule::new(src_start, dst_start, end_offset)
}

fn parse_seed_ranges(input: &str) -> Vec<SeedRange> {
    let mut ranges = Vec::<SeedRange>::new();
    let mut iter = input.split_ascii_whitespace();

    while let (Some(start), Some(len)) = (iter.next(), iter.next()) {
        let start = start.parse().unwrap();
        let len = len.parse().unwrap();
        ranges.push(SeedRange::from_len(start, len));
    }

    ranges
}

struct Rule {
    src_start: u32,
    dst_start: u32,
    // end_offset is length - 1
    end_offset: u32,
}

impl Rule {
    fn new(src_start: u32, dst_start: u32, len: u32) -> Self {
        Self {
            src_start,
            dst_start,
            end_offset: len - 1,
        }
    }
}

#[derive(Clone)]
struct SeedRange {
    start: u32,
    end: u32,
}

impl SeedRange {
    const EMPTY: Self = Self::new(1, 0);

    const fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    const fn from_len(start: u32, len: u32) -> Self {
        Self::new(start, start + len - 1)
    }

    fn is_non_empty(&self) -> bool {
        self.start <= self.end
    }
}
