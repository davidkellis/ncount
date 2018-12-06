use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Opt {
    #[structopt(parse(from_os_str))]
    paths: Vec<OsString>,
}

impl Opt {
    pub fn from_args() -> Opt {
        StructOpt::from_args()
    }

    pub fn paths(&self) -> Vec<PathBuf> {
        let mut paths = Vec::new();
        for path in &self.paths {
            append_paths(&path, &mut paths);
        }

        paths.sort();
        paths
    }
}

// Could this function get any more backward?
fn append_paths(candidate: &OsStr, buffer: &mut Vec<PathBuf>) {
    let path = Path::new(candidate);

    if let Ok(meta) = path.metadata() {
        if meta.is_file() {
            buffer.push(path.into());
        } else if meta.is_dir() {
            for path in read_file_paths(path) {
                buffer.push(path);
            }
        }
        return;
    }

    // If a path doesn't represent a valid file, it could be a glob pattern. This code path
    // will only be exercised on Windows, because Bash will stupidly expand glob patterns
    // before passing them to the program.
    if let Some(globbed_paths) = candidate.to_str().and_then(|s| glob::glob(s).ok()) {
        for path in globbed_paths.filter_map(Result::ok) {
            buffer.push(path);
        }
    }
}

fn read_file_paths(path: &Path) -> impl Iterator<Item = PathBuf> {
    use std::fs;

    fs::read_dir(path)
        .into_iter()
        .flatten()
        .filter_map(|entry| match entry {
            Err(_) => None,
            Ok(entry) => {
                if entry.metadata().ok()?.is_file() {
                    Some(entry.path())
                } else {
                    None
                }
            }
        })
}
