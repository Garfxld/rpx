use std::{env, fs, path::Path};

use anyhow::Result;
use clap::ArgMatches;
use log::debug;

use crate::{project::Project, util::paths};


pub fn exec(matches: &ArgMatches) -> Result<()> {
    let current_dir = env::current_dir().unwrap();

    let project = Project::open(&current_dir.join("example-pack"))?;

    let what = matches.get_one::<String>("path")
        .unwrap();

    let (namespace, location) = what
        .split_once("+")
        .map(|(a, b)| (
            if a.is_empty() { "minecraft" } else { a },
            paths::normalize(b)
        ))
        .expect("todo");
 

    debug!("project:   {}", project.path().to_str().unwrap());
    debug!("namespace: {}", namespace);
    debug!("path:      {}", location.to_str().unwrap());

    if !project.has_namespace(namespace) {
        project.create_namespace(namespace)?;
    }

    let assets_path = Path::new("C:\\Users\\garfxld\\Downloads\\assets");

    
    let from = assets_path.join("minecraft").join(&location);
    let to = project.get_namespace(namespace).join(&location);

    let i = copy(&from, &to)?;
    println!("+ {} files.", i);

    Ok(())
}


fn copy<P, Q>(from: P, to: Q) -> Result<i64>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let from = from.as_ref();
    let to = to.as_ref();


    if to.exists() {
        return Ok(0);
    }

    if let Some(parent) = to.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?; // create parent dirs
        }
    } else {
        return Err(anyhow::anyhow!("todo"));
    }

    let mut i = 0;
    if from.is_file() {
        fs::copy(from, to)?;
        i += 1;
    } else {
        for entry in fs::read_dir(from)? {
            let entry = entry?.path();
            let entry_to = &to.join(entry.file_name().unwrap());

            i += copy(&entry, &entry_to)?;
        }
    }
    Ok(i)
}