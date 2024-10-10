use std::path::{Component, Path, PathBuf};


// from: https://github.com/rust-lang/cargo/blob/15fbd2f607d4defc87053b8b76bf5038f2483cf4/crates/cargo-util/src/paths.rs#L84-L109
pub fn normalize<P>(path: P) -> PathBuf
where 
    P: AsRef<Path>,
{
    let path = path.as_ref();

    let mut components = path.components().peekable();
    let mut ret = if let Some(c @ Component::Prefix(..)) = components.peek().cloned() {
        components.next();
        PathBuf::from(c.as_os_str())
    } else {
        PathBuf::new()
    };

    for component in components {
        match component {
            Component::Prefix(..) => unreachable!(),
            Component::RootDir => {
                ret.push(component.as_os_str());
            }
            Component::CurDir => {}
            Component::ParentDir => {
                ret.pop();
            }
            Component::Normal(c) => {
                ret.push(c);
            }
        }
    }
    ret
}
