use std::env;

use clap::ArgMatches;

use crate::project::Project;

pub fn exec(_matches: &ArgMatches) -> anyhow::Result<()> {
    Project::open(&env::current_dir().unwrap()).unwrap();
    Ok(())
}
