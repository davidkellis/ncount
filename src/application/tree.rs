use std::path::PathBuf;
use std::{fs, vec};

pub struct Tree {
    paths: Vec<PathBuf>,
}

impl Tree {
    pub fn new<S>(patterns: &[S]) -> crate::Result<Self>
    where
        S: AsRef<str> + Into<PathBuf>,
    {
        let mut paths = Vec::new();

        for pattern in patterns.into_iter().map(AsRef::as_ref) {
            match fs::metadata(pattern) {
                Ok(meta) => {
                    if meta.is_file() {
                        paths.push(pattern.into());
                    } else {
                        let entries = walkdir::WalkDir::new(pattern)
                            .into_iter()
                            .filter_entry(|entry| entry.file_type().is_file())
                            .filter_map(|entry| entry.ok().map(|entry| entry.into_path()));

                        paths.extend(entries);
                    }
                }

                // In the event we receive a path that does not exist, we'll assume it's meant
                // as a glob rather than as a path. If not, fuck it.
                Err(_) => {
                    let entries = glob::glob(pattern)?
                        .filter_map(Result::ok)
                        .filter_map(|entry| match entry.metadata() {
                            Ok(meta) => {
                                if meta.is_file() {
                                    Some(entry)
                                } else {
                                    None
                                }
                            }
                            Err(_) => None,
                        });

                    paths.extend(entries);
                }
            }
        }

        paths.sort();
        paths.dedup();
        Ok(Tree { paths })
    }
}

impl IntoIterator for Tree {
    type Item = PathBuf;
    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.paths.into_iter()
    }
}
