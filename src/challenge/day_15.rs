use crate::utils::Bytes;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(input[0]
        .split(',')
        .map(|step| hash(step.as_bytes()))
        .sum::<usize>())
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    // [Lense; 6] is allocated directly on the stack
    // so it's is faster than Vec<Lense>, but required more logic
    let mut boxes = [[Lense::default(); 6]; 256];

    // .as_bytes().split() avoids the performance penalty to variable-width UTF-8 chars
    for step in input[0].as_bytes().split(|char| *char == b',') {
        let index = step
            .iter()
            .rposition(|char| !char.is_ascii_alphanumeric())
            .unwrap();

        let label = &step[..index];
        let id = id(label);
        let operation = step[index];

        let slots = &mut boxes[hash(label)];

        if operation == b'-' {
            let index = match slots.iter().position(|lense| lense.id == id) {
                Some(offset) => offset,
                None => continue,
            };

            let next = index + 1;

            let length = match slots[next..].iter().position(|lense| lense.id == 0) {
                Some(offset) => next + offset,
                None => slots.len(),
            };

            // shift the lenses and clear the last lense
            if next < length {
                // equal to slots[index..length].rotate_left(1)
                // copy_within uses memmove which yield a small performance improvement
                slots.copy_within(next..length, index);
                slots[length - 1] = Lense::default();
            } else {
                slots[index] = Lense::default();
            }
        } else {
            let focal_length = &step[index + 1..];

            let lense = slots
                .iter_mut()
                .find(|lense| lense.id == id || lense.id == 0)
                .unwrap();

            *lense = Lense { id, focal_length };
        }
    }

    let focusing_power = boxes
        .into_iter()
        .enumerate()
        .filter(|(_, slots)| slots[0].id != 0)
        .flat_map(|(i, slots)| {
            let box_multiplier = i + 1;
            // skipping over empty slots doesn't yield a performance improvement due to the additional branches
            slots.into_iter().enumerate().map(move |(i, lense)| {
                let slot_multiplier = i + 1;
                lense.focal_length.parse_dec::<usize>() * slot_multiplier * box_multiplier
            })
        })
        .sum::<usize>();

    Ok(focusing_power)
}

// assumes the label isn't longer than 8 bytes
fn id(value: &[u8]) -> u64 {
    let mut bytes = [0u8; 8];
    bytes[0..value.len()].copy_from_slice(value);
    u64::from_ne_bytes(bytes)
}

fn hash(value: &[u8]) -> usize {
    value
        .iter()
        .fold(0, |acc, char| (acc + *char as usize) * 17 % 256)
}

#[derive(Default, Copy, Clone)]
struct Lense<'a> {
    id: u64,
    focal_length: &'a [u8],
}
