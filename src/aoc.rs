use crate::day::Day;
use anyhow::Context;
use std::{
    borrow::Cow,
    path::{Path, PathBuf},
    sync::{Mutex, MutexGuard, OnceLock},
};

const INPUTS_DIR: &str = "inputs";
static CACHE: OnceLock<Mutex<[Option<&'static [&'static str]>; 25]>> = OnceLock::new();

pub fn get(day: Day) -> anyhow::Result<&'static [&'static str]> {
    if let Some(cached) = cache()[day.into_index()] {
        return Ok(cached);
    }

    let response = Box::leak(laod_inputs(day)?.into_boxed_str());
    let lines = Box::leak(response.lines().collect());
    cache()[day.into_index()] = Some(lines);
    Ok(lines)
}

fn cache() -> MutexGuard<'static, [Option<&'static [&'static str]>; 25]> {
    CACHE.get_or_init(Default::default).lock().unwrap()
}

fn laod_inputs(day: Day) -> anyhow::Result<String> {
    let parent_dir = match std::env::var("INPUTS") {
        Ok(path) => Cow::Owned(PathBuf::from(path)),
        Err(_) => Cow::Borrowed(Path::new(INPUTS_DIR)),
    };

    let path = {
        let mut path = parent_dir.to_path_buf();
        path.push(format!("{:02}.txt", day));
        path
    };

    if let Ok(response) = std::fs::read_to_string(&path) {
        println!("{}", path.display());
        return Ok(response);
    }

    let session = std::env::var("SESSION").context("SESSION is not defined")?;

    let response = reqwest::blocking::Client::default()
        .get(format!("https://adventofcode.com/2023/day/{day}/input"))
        .header("cookie", format!("session={}", session))
        .send()?
        .error_for_status()?
        .text()?;

    if create_parent_dir(&parent_dir) {
        save_inputs(&response, &path);
    }

    Ok(response)
}

fn create_parent_dir(parent_dir: &Path) -> bool {
    if parent_dir.exists() {
        return true;
    }

    let err = match std::fs::create_dir(parent_dir) {
        Ok(_) => return true,
        Err(err) => err,
    };

    eprintln!(
        "Failed to create inputs directory as {}: {err}",
        parent_dir.display()
    );

    false
}

fn save_inputs(content: &str, path: &Path) {
    // Ignore, it'll be retired when this is loaded the next time
    if let Err(err) = std::fs::write(path, content) {
        eprintln!("Failed to write input file at {}: {err}", path.display());
    }
}
