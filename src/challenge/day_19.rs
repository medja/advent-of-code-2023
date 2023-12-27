use crate::utils::{Bytes, IndexMapBuilder};

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut parts = input.split(|line| line.is_empty());
    let workflows = Parser::parse(parts.next().unwrap());

    let mut sum = 0;

    for input in parts.next().unwrap() {
        let part = parse_part(input);

        if validate_part(&part, &workflows) {
            sum += part.iter().map(|value| *value as usize).sum::<usize>();
        }
    }

    Ok(sum)
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let workflows = Parser::parse(input.split(|line| line.is_empty()).next().unwrap());
    Ok(count_accepted(0, [Range::FULL; 4], 0, &workflows))
}

fn validate_part(part: &[u16; 4], workflows: &[Workflow]) -> bool {
    let mut index = 0;

    loop {
        let action = workflows[index]
            .rules
            .iter()
            .find(|rule| {
                rule.value != 0
                    && match rule.comparison {
                        Comparison::Less => part[rule.field as usize] < rule.value,
                        Comparison::Greater => part[rule.field as usize] > rule.value,
                    }
            })
            .map(|rule| rule.action)
            .unwrap_or(workflows[index].default);

        match action {
            Action::Accept => {
                break true;
            }
            Action::Reject => {
                break false;
            }
            Action::Workflow(next) => {
                index = next as usize;
            }
        }
    }
}

fn count_accepted(
    index: usize,
    mut ranges: [Range; 4],
    mut accepted: u64,
    workflows: &[Workflow],
) -> u64 {
    for rule in workflows[index]
        .rules
        .iter()
        .take_while(|rule| rule.value != 0)
    {
        let field = rule.field as usize;
        let mut matched_range = ranges[field].clone();

        match rule.comparison {
            Comparison::Less => matched_range.end = matched_range.end.min(rule.value - 1),
            Comparison::Greater => matched_range.start = matched_range.start.max(rule.value + 1),
        };

        if matched_range.start > matched_range.end {
            // no match
            continue;
        } else if matched_range == ranges[field] {
            // full match
            return match rule.action {
                Action::Accept => accepted + count_combinations(&ranges),
                Action::Reject => accepted,
                Action::Workflow(next) => {
                    count_accepted(next as usize, ranges, accepted, workflows)
                }
            };
        }

        // partial match
        let mut remaining_range = ranges[field].clone();

        if matched_range.start == remaining_range.start {
            remaining_range.start = matched_range.end + 1;
        } else {
            remaining_range.end = matched_range.start - 1;
        }

        ranges[field] = matched_range;

        match rule.action {
            Action::Accept => accepted += count_combinations(&ranges),
            Action::Reject => {}
            Action::Workflow(next) => {
                accepted += count_accepted(next as usize, ranges.clone(), 0, workflows)
            }
        }

        ranges[field] = remaining_range;
    }

    match workflows[index].default {
        Action::Accept => accepted + count_combinations(&ranges),
        Action::Reject => accepted,
        Action::Workflow(index) => count_accepted(index as usize, ranges, accepted, workflows),
    }
}

fn count_combinations(ranges: &[Range; 4]) -> u64 {
    ranges.iter().map(Range::count).product::<u64>()
}

#[derive(Clone, Default)]
struct Workflow {
    rules: [Rule; 3],
    default: Action,
}

#[derive(Copy, Clone, Default)]
struct Rule {
    field: u8,
    comparison: Comparison,
    value: u16,
    action: Action,
}

#[derive(Copy, Clone, Default)]
enum Comparison {
    #[default]
    Less,
    Greater,
}

#[derive(Copy, Clone, Default)]
enum Action {
    #[default]
    Accept,
    Reject,
    Workflow(u16),
}

#[derive(Clone, Eq, PartialEq)]
struct Range {
    start: u16,
    end: u16,
}

impl Range {
    const FULL: Self = Self {
        start: 1,
        end: 4000,
    };

    fn count(&self) -> u64 {
        (self.end - self.start) as u64 + 1
    }
}

struct Parser<'a>(IndexMapBuilder<&'a [u8], Workflow>);

impl<'a> Parser<'a> {
    fn parse(input: &'a [&'a str]) -> Vec<Workflow> {
        let mut parser = Self(IndexMapBuilder::with_capacity(input.len()));
        parser.0.reserve(b"in");

        for input in input {
            parser.parse_workflow(input.as_bytes());
        }

        parser.0.build()
    }

    fn parse_workflow(&mut self, input: &'a [u8]) {
        let start = input.iter().position(|char| *char == b'{').unwrap();
        let end = input.iter().rposition(|char| *char == b',').unwrap();

        let id = self.0.find_index(&input[..start]);
        let rules = self.parse_rules(&input[start + 1..end]);
        let default = self.parse_action(&input[end + 1..input.len() - 1]);

        self.0[id] = Workflow { rules, default };
    }

    fn parse_rules(&mut self, input: &'a [u8]) -> [Rule; 3] {
        let mut rules = [Rule::default(); 3];

        for (index, input) in input.split(|char| *char == b',').enumerate() {
            let i = input.iter().rposition(|char| *char == b':').unwrap();

            let field = parse_field_index(input[0]);
            let comparison = parse_comparison(input[1]);
            let value = input[2..i].parse_dec();
            let action = self.parse_action(&input[i + 1..]);

            rules[index] = Rule {
                field,
                comparison,
                value,
                action,
            };
        }

        rules
    }

    fn parse_action(&mut self, value: &'a [u8]) -> Action {
        match value {
            [b'A'] => Action::Accept,
            [b'R'] => Action::Reject,
            _ => Action::Workflow(self.0.find_index(value) as u16),
        }
    }
}

// parse "{x=787,m=2655,a=1222,s=2876}" into an array with 4 numbers
fn parse_part(input: &str) -> [u16; 4] {
    let mut iter = input
        .as_bytes()
        .split(|char| !char.is_ascii_digit())
        .filter(|value| !value.is_empty())
        .map(|value| value.parse_dec());

    [
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
    ]
}

fn parse_field_index(value: u8) -> u8 {
    match value {
        b'x' => 0,
        b'm' => 1,
        b'a' => 2,
        b's' => 3,
        _ => unreachable!(),
    }
}

fn parse_comparison(value: u8) -> Comparison {
    match value {
        b'<' => Comparison::Less,
        b'>' => Comparison::Greater,
        _ => unreachable!(),
    }
}
