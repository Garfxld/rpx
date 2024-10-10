use clap::ArgMatches;

pub fn exec(_matches: &ArgMatches) -> anyhow::Result<()> {
    println!("info");
    Ok(())
}
