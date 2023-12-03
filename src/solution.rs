use crate::{
    aoc,
    day::{Day, Part},
};
use std::time::{Duration, Instant};

pub struct Solution {
    pub day: Day,
    pub part: Part,
    pub name: &'static str,
    pub result: String,
    pub duration: Duration,
}

pub trait Challenge {
    fn solve(
        &self,
        day: Day,
        part: Part,
        name: &'static str,
        input: &'static [&'static str],
    ) -> anyhow::Result<Solution>;
}

impl<F, R> Challenge for F
where
    F: Fn(&'static [&'static str]) -> anyhow::Result<R>,
    R: std::fmt::Display,
{
    fn solve(
        &self,
        day: Day,
        part: Part,
        name: &'static str,
        input: &'static [&'static str],
    ) -> anyhow::Result<Solution> {
        let start = Instant::now();
        let output = self(input)?;
        let duration = start.elapsed();
        let result = output.to_string();

        Ok(Solution {
            day,
            part,
            name,
            result,
            duration,
        })
    }
}

pub struct Challenges([Option<Parts>; 25]);

impl Challenges {
    pub fn solve(&self, day: Day, part: Part) -> anyhow::Result<Solution> {
        let parts = match self.0[day.into_index()] {
            Some(ref parts) => parts,
            None => anyhow::bail!("Day is not defined"),
        };

        let challenge = match part {
            Part::A => Some(&parts.part_a),
            Part::B => parts.part_b.as_ref(),
        };

        let challenge = match challenge {
            Some(challenge) => challenge,
            None => anyhow::bail!("Part is not solved"),
        };

        challenge.solve(day, part, parts.name, aoc::get(day)?)
    }

    #[doc(hidden)]
    pub fn new() -> Self {
        Self(std::array::from_fn(|_| None))
    }

    #[doc(hidden)]
    pub fn insert(&mut self, day: usize, parts: Parts) {
        if let Ok(day) = Day::try_from(day) {
            self.0[day.into_index()] = Some(parts);
        }
    }
}

pub struct Parts {
    name: &'static str,
    part_a: Box<dyn Challenge>,
    part_b: Option<Box<dyn Challenge>>,
}

impl Parts {
    #[doc(hidden)]
    pub fn new(
        name: &'static str,
        part_a: Box<dyn Challenge>,
        part_b: Option<Box<dyn Challenge>>,
    ) -> Self {
        Self {
            name,
            part_a,
            part_b,
        }
    }
}

macro_rules! build {
    ($(day($day:expr, $name:expr, $part_a:expr $(,$part_b:expr)? $(,)?)),* $(,)?) => {
        let mut challenges = $crate::solution::Challenges::new();

        $({
            #[allow(unused_assignments, unused_mut)]
            let mut part_b = Option::<Box<dyn $crate::solution::Challenge>>::None;
            $(part_b = Some(Box::new($part_b));)?
            let parts = $crate::solution::Parts::new($name, Box::new($part_a), part_b);
            challenges.insert($day, parts);
        })*;

        challenges
    };
}

pub(crate) use build;
