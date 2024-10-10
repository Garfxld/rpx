mod command;
mod project;
mod prompt;
mod util;

use clap::{Arg, Command};


fn main() -> anyhow::Result<()> {
    env_logger::init();

    let command = get_command();
    let matches = command.get_matches();

    match matches.subcommand() {
        Some(("new", new_matches)) => {
            command::new::exec(&new_matches)?;
        }
        Some(("add", add_matches)) => {
            command::add::exec(&add_matches)?;
        }
        Some(("info", info_matches)) => {
            command::info::exec(&info_matches)?;
        }
        Some(("build", build_matches)) => {
            command::build::exec(&build_matches)?;
        }
        Some(("tree", tree_matches)) => {
            command::tree::exec(&tree_matches)?;
        }
        _ => unreachable!(), // should never be reachable
    }

    Ok(())
}


fn get_command() -> Command {
    Command::new("rpx")
        .version("v0.1")
        .about("about rpx")
        .subcommand_required(true)
        .subcommand(
            Command::new("new")
                .about("Create a new resouce pack")
                .arg(
                    Arg::new("name")
                        .help("The name of the resouce pack")
                        .num_args(1),
                ),
        )
        .subcommand(
            Command::new("init")
                .about("Initializes a directory to ")
                .arg(
                    Arg::new("path")
                        .help("The path to initialize the resource pack")
                        .required(true)
                        .num_args(1),
                ),
        )
        .subcommand(
            Command::new("add")
                .about("todo")
                .arg(
                    Arg::new("path")
                        .help("todo")
                        .required(true)
                        .num_args(1)
                ),
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
                        .num_args(1),
                ),
        )
        .subcommand(
            Command::new("tree")
                .about("Prints the resource pack tree")
                .arg(
                    Arg::new("path")
                        .help("todo")
                        .num_args(1)
                    ),
        )
}
