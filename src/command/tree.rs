use std::{env, fs, path::Path};

use clap::ArgMatches;
use crossterm::style::Stylize;

pub fn exec(matches: &ArgMatches) -> anyhow::Result<()> {
    let fallback = env::current_dir().unwrap().display().to_string();
    let path = matches
        .get_one::<String>("path")
        .or(Some(&fallback))
        .unwrap();

    visit_dir(&Path::new(path), "", 0).unwrap();
    Ok(())
}


fn visit_dir(path: &Path, prefix: &str, level: usize) -> anyhow::Result<()> {
    if !path.is_dir() {
        return Ok(());
    }
    if level == 0 {
        println!("{} {}", file_name(path).bold(), "(root)".bold());
    }

    let mut entries = fs::read_dir(path)?
        .filter_map(Result::ok)
        .map(|e| e.path())
        .collect::<Vec<_>>();

    entries.sort_by_key(|f| f.is_dir());

    let mut iter = entries.iter().peekable();
    while let Some(entry) = iter.next() {
        let (current_prefix, new_prefix) = match iter.peek() {
            Some(_) => ("├─", "│  "),
            None => ("└─", "   "),
        };
        let file_name = if entry.is_dir() {
            file_name(&entry).bold()
        } else {
            file_name(&entry).reset()
        };

        println!("{}{} {}", prefix, current_prefix, file_name);

        visit_dir(&entry, &format!("{}{}", prefix, new_prefix), level + 1)?;
    }

    Ok(())
}

fn file_name(path: &Path) -> String {
    path.file_name().unwrap().to_str().unwrap().to_string()
}