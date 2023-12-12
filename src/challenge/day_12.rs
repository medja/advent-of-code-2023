pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut solver = Solver::default();

    Ok(input
        .iter()
        .map(|line| solver.solve(line, false))
        .sum::<u64>())
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut solver = Solver::default();

    Ok(input
        .iter()
        .map(|line| solver.solve(line, true))
        .sum::<u64>())
}

#[derive(Default)]
struct Solver {
    springs: Vec<Condition>,
    queue: Vec<usize>,
    min_lengths: Vec<usize>,
    last_damaged: usize,
    cache: Vec<u64>,
}

impl Solver {
    fn solve(&mut self, input: &str, unfold: bool) -> u64 {
        self.parse_input(input);

        if unfold {
            self.unfold_springs();
            self.unfold_queue();
        }

        self.last_damaged = self
            .springs
            .iter()
            .rposition(|condition| *condition == Condition::Damaged)
            .unwrap_or_default();

        self.initialize_min_lengths();
        self.initialize_cache();
        self.find_arrangements(0, 0)
    }

    fn parse_input(&mut self, input: &str) {
        let (conditions, queue) = input.split_once(' ').unwrap();

        self.springs.clear();
        self.springs.extend(conditions.bytes().map(Condition::from));

        let queue = queue
            .split(',')
            .map(|count| count.parse::<usize>().unwrap());

        self.queue.clear();
        self.queue.extend(queue);
    }

    fn unfold_springs(&mut self) {
        let length = self.springs.len();
        self.springs.resize(length * 5 + 4, Condition::Unknown);

        let (original, extended) = self.springs.split_at_mut(length);

        // step by `length + 1` to skip the joining unknown spring
        for start in (1..extended.len()).step_by(length + 1) {
            extended[start..start + length].copy_from_slice(original);
        }
    }

    fn unfold_queue(&mut self) {
        let length = self.queue.len();
        self.queue.resize(length * 5, 0);

        let (original, extended) = self.queue.split_at_mut(length);

        for start in (0..extended.len()).step_by(length) {
            extended[start..start + length].copy_from_slice(original);
        }
    }

    // determines the minimum number of springs required by the remaining queue
    fn initialize_min_lengths(&mut self) {
        self.min_lengths.resize(self.queue.len(), 0);

        let mut min_length = *self.queue.last().unwrap();
        *self.min_lengths.last_mut().unwrap() = min_length;

        for (i, size) in self.queue.iter().enumerate().rev().skip(1) {
            min_length += *size + 1;
            self.min_lengths[i] = min_length;
        }
    }

    fn initialize_cache(&mut self) {
        let length = self.springs.len() * self.queue.len() + 1;

        if length <= self.cache.len() {
            self.cache.truncate(length);
            self.cache.fill(u64::MAX);
        } else {
            self.cache.fill(u64::MAX);
            self.cache.resize(length, u64::MAX);
        }
    }

    fn find_arrangements(&mut self, spring_offset: usize, queue_offset: usize) -> u64 {
        if queue_offset == self.queue.len() {
            return (spring_offset > self.last_damaged) as u64;
        }

        if spring_offset >= self.springs.len() {
            return 0;
        }

        let id = spring_offset + queue_offset * self.springs.len();
        let cached = self.cache[id];

        if cached != u64::MAX {
            return cached;
        }

        let count = self.count_arrangements(spring_offset, queue_offset);
        self.cache[id] = count;
        count
    }

    fn count_arrangements(&mut self, spring_offset: usize, queue_offset: usize) -> u64 {
        let size = self.queue[queue_offset];
        let min_length = self.min_lengths[queue_offset];

        let (position, mendatory) = match self.find_next_position(spring_offset, size, min_length) {
            Some(result) => result,
            None => return 0,
        };

        let count = if mendatory {
            0
        } else {
            self.find_arrangements(position + 1, queue_offset)
        };

        count + self.find_arrangements(position + size + 1, queue_offset + 1)
    }

    fn find_next_position(
        &self,
        start: usize,
        size: usize,
        min_length: usize,
    ) -> Option<(usize, bool)> {
        let end = self.springs.len() - min_length + 1;

        if end < start {
            return None;
        }

        for (index, condition) in self.springs[start..end].iter().enumerate() {
            let mendatory = match condition {
                Condition::Operational => continue,
                Condition::Damaged => true,
                Condition::Unknown => false,
            };

            let position = index + start;

            if self.matches_pattern(position, size) {
                return Some((position, mendatory));
            } else if mendatory {
                break;
            }
        }

        None
    }

    fn matches_pattern(&self, position: usize, size: usize) -> bool {
        if matches!(self.springs.get(position + size), Some(Condition::Damaged)) {
            return false;
        }

        // matches_pattern is only used when self.springs[position] isn't be operational
        !self.springs[position + 1..position + size].contains(&Condition::Operational)
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

impl From<u8> for Condition {
    fn from(value: u8) -> Self {
        match value {
            b'.' => Self::Operational,
            b'#' => Self::Damaged,
            b'?' => Self::Unknown,
            _ => unreachable!(),
        }
    }
}
