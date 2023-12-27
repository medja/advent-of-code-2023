use challenge::challenges;
use day::{Day, Part};
use solution::Challenges;
use std::io::{BufRead, Write};

mod aoc;
mod challenge;
mod day;
mod solution;
mod utils;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

fn main() -> anyhow::Result<()> {
    if std::env::args().count() > 1 {
        run(std::env::args().skip(1).map(Ok), false)
    } else {
        run(std::io::stdin().lock().lines(), true)
    }
}

fn run(args: impl Iterator<Item = std::io::Result<String>>, prompt: bool) -> anyhow::Result<()> {
    if prompt {
        print!("> ");
        _ = std::io::stdout().flush();
    }

    let challenges = challenges();

    for arg in args {
        solve(&arg?, &challenges);

        if prompt {
            println!();
            print!("> ");
            _ = std::io::stdout().flush();
        }
    }

    Ok(())
}

fn solve(arg: &str, challenges: &Challenges) {
    let (day, part) = match parse_arg(arg) {
        Ok(Some((day, part))) => (day, part),
        Ok(None) => return,
        Err(err) => {
            println!("Cannot parse '{arg}': {}", err);
            return;
        }
    };

    match challenges.solve(day, part) {
        Ok(solution) => println!(
            "Day {}: {} (Part {}): {} (duration = {:?})",
            solution.day, solution.name, solution.part, solution.result, solution.duration
        ),
        Err(err) => println!("Failed to solve Day {day} Part {part}: {err}"),
    }
}

fn parse_arg(input: &str) -> anyhow::Result<Option<(Day, Part)>> {
    let input = input.trim();

    if input.is_empty() {
        return Ok(None);
    }

    let index = input.find(|c: char| !c.is_ascii_digit());

    let (day, part) = match index {
        Some(index) => (&input[..index], &input[index..]),
        None => (input, ""),
    };

    Ok(Some((day.parse()?, part.trim().parse()?)))
}
