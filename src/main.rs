mod prompt;

use std::{fs::{self, File}, path::Path};

use clap::{Arg, Command};
use prompt::Prompt;

use serde_json::{json, to_writer_pretty};



fn main() {
    let matches = Command::new("rpx")
        .version("v0.1")
        .about("about rpx")
        .subcommand_required(true)
        .subcommand(
            Command::new("new")
                .about("Create a new resouce pack")
                .arg(
                    Arg::new("name")
                        .help("The name of the resouce pack")
                        .num_args(1)
                )
        )
        .subcommand(
            Command::new("info")
                .about("Jump to the docs")
        )
        .subcommand(
            Command::new("build")
                .about("Builds the resource pack")
                .arg(
                    Arg::new("path")
                        .help("The path of the resource pack to build")
                        .required(true)
                        .num_args(1)
                )
        )
        .get_matches();

    match matches.subcommand() {
        Some(("new", new_matches)) => {

            let name = new_matches.get_one::<String>("name")
                .unwrap_or(
                    &Prompt::new("What should the new project be called?\n    Note", "blah-blah")
                        .prompt()
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

            println!("directory: {}, name: {}, version: {}", directory.to_str().unwrap(), name, version);
        }
        Some(("info", _)) => {
            println!("info");
        }
        Some(("build", _)) => {
            println!("build");
        }
        _ => unreachable!()
    }

}

