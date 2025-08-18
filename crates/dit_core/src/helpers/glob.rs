use crate::helpers::path_to_string;
use crate::errors::DitResult;
use crate::errors::OtherError::{GlobBuildError, GlobPatternError};
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use std::path::Path;
use std::sync::Arc;
use ignore::overrides::OverrideBuilder;
use ignore::{DirEntry, WalkBuilder};


/// Builds a [`Gitignore`] given a file path
pub fn ignore_from_file(root: &Path, ignore_file: &Path) -> DitResult<Gitignore> {
    let mut builder = GitignoreBuilder::new(root);
    builder.add(ignore_file);
    builder
        .build()
        .map_err(|_| GlobBuildError(path_to_string(ignore_file)).into())
}


/// Accepts a collection of globs and returns the files
pub fn expand_globs<I>(root: &Path, globs: I, ignore: Arc<Gitignore>) -> DitResult<Vec<DirEntry>>
where I: Iterator,
    I::Item: AsRef<str>,
{
    let mut ob = OverrideBuilder::new(root);
    for g in globs {
        let g = g.as_ref();
        ob.add(g).map_err(|_| GlobPatternError(g.to_string()))?;
    }
    let overrides = ob.build()
        .map_err(|_| GlobBuildError(path_to_string(root)))?;

    let ignore_for_dirs = ignore.clone();

    let walker = WalkBuilder::new(root)
        .overrides(overrides)
        .hidden(false)
        .git_ignore(false).git_global(false).git_exclude(false)
        .standard_filters(false)
        .filter_entry(move |e| {
            let is_dir = e.file_type().map(|t| t.is_dir()).unwrap_or(true);
            if is_dir {
                !ignore_for_dirs.matched_path_or_any_parents(e.path(), true).is_ignore()
            } else {
                true
            }
        })
        .build();

    let files = walker
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().map(|ft| ft.is_file()).unwrap_or_else(|| e.path().is_file()))
        .filter(move |e| !ignore.matched_path_or_any_parents(e.path(), false).is_ignore())
        .collect();

    Ok(files)
}
