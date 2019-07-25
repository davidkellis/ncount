use std::path::PathBuf;

pub struct Tree {
    paths: glob::Paths,
}

impl Tree {
    pub fn new(pattern: &str) -> crate::Result<Self> {
        Ok(Tree {
            paths: glob::glob(pattern)?,
        })
    }
}

// WARNING: all this iterator crap below here is kinda pointless.

pub struct TreeIter<'tree> {
    tree: &'tree mut Tree,
}

impl Iterator for TreeIter<'_> {
    type Item = PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Ok(path) = self.tree.paths.next()? {
                return Some(path)
            }
        }
    }
}

impl<'tree> IntoIterator for &'tree mut Tree {
    type Item = PathBuf;
    type IntoIter = TreeIter<'tree>;

    fn into_iter(self) -> Self::IntoIter {
        TreeIter { tree: self }
    }
}

pub struct TreeIntoIter {
    paths: glob::Paths,
}

impl Iterator for TreeIntoIter {
    type Item = PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Ok(path) = self.paths.next()? {
                return Some(path);
            }
        }
    }
}

impl IntoIterator for Tree {
    type Item = PathBuf;
    type IntoIter = TreeIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        TreeIntoIter { paths: self.paths }
    }
}
