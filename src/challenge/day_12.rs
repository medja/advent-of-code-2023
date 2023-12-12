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
    cache: Vec<u64>,
}

impl Solver {
    fn solve(&mut self, input: &str, unfold: bool) -> u64 {
        self.parse_input(input);

        if unfold {
            self.unfold_springs();
            self.unfold_queue();
        }

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
            return !self.contains_damaged(spring_offset) as u64;
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

        let (position, mendatory) = match self.find_next_position(spring_offset, size) {
            Some(result) => result,
            None => return 0,
        };

        let mut count = self.find_arrangements(position + size + 1, queue_offset + 1);

        if !mendatory {
            count += self.find_arrangements(position + 1, queue_offset);
        }

        count
    }

    fn find_next_position(&self, offset: usize, size: usize) -> Option<(usize, bool)> {
        if offset + size > self.springs.len() {
            return None;
        }

        for (index, condition) in self.springs[offset..self.springs.len() - size + 1]
            .iter()
            .enumerate()
        {
            let mendatory = match condition {
                Condition::Operational => continue,
                Condition::Damaged => true,
                Condition::Unknown => false,
            };

            let position = index + offset;

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

    fn contains_damaged(&self, spring_offset: usize) -> bool {
        spring_offset < self.springs.len()
            && self.springs[spring_offset..].contains(&Condition::Damaged)
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
