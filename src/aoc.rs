use crate::day::Day;
use anyhow::Context;
use std::sync::{Mutex, MutexGuard, OnceLock};

static CACHE: OnceLock<Mutex<[Option<&'static [&'static str]>; 25]>> = OnceLock::new();

pub async fn get(day: Day) -> anyhow::Result<&'static [&'static str]> {
    if let Some(cached) = cache()[day.into_index()] {
        return Ok(cached);
    }

    let session = std::env::var("SESSION").context("SESSION is not defined")?;

    let response = reqwest::Client::default()
        .get(format!("https://adventofcode.com/2023/day/{day}/input"))
        .header("cookie", format!("session={}", session))
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;

    let mut cache = cache();

    if let Some(cached) = cache[day.into_index()] {
        return Ok(cached);
    }

    let response = Box::leak(response.into_boxed_str());
    let lines = Box::leak(response.lines().collect::<Box<[_]>>());
    cache[day.into_index()] = Some(lines);
    Ok(lines)
}

fn cache() -> MutexGuard<'static, [Option<&'static [&'static str]>; 25]> {
    CACHE.get_or_init(Default::default).lock().unwrap()
}
