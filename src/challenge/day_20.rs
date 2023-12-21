use gcd::Gcd;
use rustc_hash::FxHashMap;
use std::{
    collections::{hash_map::Entry, VecDeque},
    ops::BitOrAssign,
};

const RX_INDEX: usize = 1;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut queue = VecDeque::new();
    let mut connections = parse(input);

    let mut low_count = 0;
    let mut high_count = 0;

    for _ in 0..1000 {
        let (l, h, _) = simulate(&mut connections, &mut queue, 0);
        low_count += l;
        high_count += h;
    }

    Ok(low_count * high_count)
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut queue = VecDeque::new();
    let mut connections = parse(input);

    let last_conjunction = connections
        .iter()
        .position(|connection| connection.children.has(RX_INDEX))
        .unwrap();

    let required = match connections[last_conjunction].module {
        Module::Conjunction { required, .. } => required,
        _ => unreachable!(),
    };

    let mut seen = BitSet::default();

    // Visualizing the problem shows that the modules form 4 "loops".
    // Exactly one of the modules of a loop receives pulses from the broadcaster and exactly one of
    // the modules emits a pulse to the conjunction module before rx.
    // Because of how the loops are structured, they emit a high pulse on exact intervals.

    // This solution assumes that rx receives pulses from a single conjunction module and that
    // its input modules emit high pulses on exact interval. It doesn't assume 4 loops as I could
    // not confirm that when implementing the solution.

    let mut step = 0u64;
    let mut solution = 1u64;

    loop {
        step += 1;

        let (_, _, sources) = simulate(&mut connections, &mut queue, last_conjunction);

        if sources.is_empty() {
            continue;
        }

        seen |= sources;
        solution = solution * step / solution.gcd(step);

        if seen == required {
            break Ok(solution);
        }
    }
}

fn simulate(
    connections: &mut [Connection],
    queue: &mut VecDeque<Pulse>,
    monitor: usize,
) -> (usize, usize, BitSet) {
    queue.push_front(Pulse::default());

    let mut low_count = 0;
    let mut hight_count = 0;
    let mut sources = BitSet::default();

    while let Some(pulse) = queue.pop_front() {
        if pulse.state {
            hight_count += 1;
        } else {
            low_count += 1;
        }

        let state = match &mut connections[pulse.destination].module {
            Module::Broadcaster => pulse.state,
            Module::FlipFlop { .. } if pulse.state => continue,
            Module::FlipFlop { state } => {
                *state = !*state;
                *state
            }
            Module::Conjunction { active, required } => {
                if pulse.state && pulse.destination == monitor {
                    sources = sources.set(pulse.source);
                }

                *active = if pulse.state {
                    active.set(pulse.source)
                } else {
                    active.unset(pulse.source)
                };

                active != required
            }
        };

        for child in connections[pulse.destination].children {
            queue.push_back(Pulse {
                source: pulse.destination,
                destination: child,
                state,
            });
        }
    }

    (low_count, hight_count, sources)
}

fn parse(input: &[&str]) -> Vec<Connection> {
    let mut ids = FxHashMap::<&[u8], usize>::default();
    ids.insert(b"broadcaster", 0);
    ids.insert(b"rx", RX_INDEX);

    let mut next_id = 2;
    let mut connections = vec![Connection::default(); input.len() + 1]; // rx doesn't have its own row

    for line in input.iter() {
        let mut parts = line.as_bytes().split(|char| *char == b' ');
        let source = parts.next().unwrap();
        parts.next(); // skip "->"

        let module = match source[0] {
            b'b' => Module::Broadcaster,
            b'%' => Module::FlipFlop { state: false },
            b'&' => Module::Conjunction {
                active: BitSet::default(),
                required: BitSet::default(),
            },
            _ => unreachable!(),
        };

        let id = get_id(source, &mut next_id, &mut ids);

        let children = parts
            .map(|child| get_id(child, &mut next_id, &mut ids))
            .fold(BitSet::default(), |set, id| set.set(id));

        connections[id] = Connection { module, children };
    }

    for i in 0..connections.len() {
        for child in connections[i].children {
            if let Module::Conjunction { required, .. } = &mut connections[child].module {
                *required = required.set(i);
            }
        }
    }

    connections
}

fn get_id<'a>(value: &'a [u8], next_id: &mut usize, ids: &mut FxHashMap<&'a [u8], usize>) -> usize {
    let value = if !value[0].is_ascii_lowercase() {
        &value[1..]
    } else if !value[value.len() - 1].is_ascii_lowercase() {
        &value[..value.len() - 1]
    } else {
        value
    };

    let entry = match ids.entry(value) {
        Entry::Occupied(entry) => return *entry.get(),
        Entry::Vacant(entry) => entry,
    };

    let id = *next_id;
    *next_id += 1;
    entry.insert(id);
    id
}

#[derive(Clone, Eq, PartialEq, Default)]
struct Connection {
    module: Module,
    children: BitSet,
}

#[derive(Clone, Eq, PartialEq, Default)]
enum Module {
    #[default]
    Broadcaster,
    FlipFlop {
        state: bool,
    },
    Conjunction {
        active: BitSet,
        required: BitSet,
    },
}

#[derive(Default)]
struct Pulse {
    source: usize,
    destination: usize,
    state: bool,
}

#[derive(Copy, Clone, Eq, PartialEq, Default)]
struct BitSet(u64);

impl BitSet {
    fn is_empty(self) -> bool {
        self.0 == 0
    }

    fn has(self, value: usize) -> bool {
        self.0 & (1 << value) != 0
    }

    fn set(self, value: usize) -> Self {
        Self(self.0 | (1 << value))
    }

    fn unset(self, value: usize) -> Self {
        Self(self.0 & !(1 << value))
    }
}

impl BitOrAssign for BitSet {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0
    }
}

impl IntoIterator for BitSet {
    type Item = usize;

    type IntoIter = BitSetIterator;

    fn into_iter(self) -> Self::IntoIter {
        BitSetIterator(self.0)
    }
}

struct BitSetIterator(u64);

impl Iterator for BitSetIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }

        let lowest_bit = self.0 & 0u64.wrapping_sub(self.0);
        let item = lowest_bit.trailing_zeros();
        self.0 ^= lowest_bit;

        Some(item as usize)
    }
}