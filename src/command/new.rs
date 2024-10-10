use std::{fs::{self, File}, path::Path};

use clap::ArgMatches;
use serde_json::{json, to_writer_pretty};

use crate::prompt::Prompt;


pub fn exec(matches: &ArgMatches) -> anyhow::Result<()> {
    let name = matches
    .get_one::<String>("name")
    .unwrap_or(
        &Prompt::new(
            "What should the new project be called?\n    Note",
            "example-pack",
        )
        .prompt(),
    )
    .to_owned();
let directory = Path::new(&name);

let version = Prompt::new("What is the format of the project?", "34")
    .numbers_only()
    .prompt();

if directory.exists() {
    panic!("directory already exists");
}

fs::create_dir(&directory).unwrap();

let json = json!({
    "pack": {
        "pack_format": version.parse::<i32>().unwrap(),
        "description": "",
    },
});
let file = File::create(directory.join("pack.mcmeta")).unwrap();
to_writer_pretty(file, &json).unwrap();

fs::create_dir(directory.join("assets")).unwrap();

println!(
    "directory: {}, name: {}, version: {}",
    directory.to_str().unwrap(),
    name,
    version
);

    Ok(())
}