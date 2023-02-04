use git2::{Repository, StatusOptions};
use super::{Context, Module, ModuleConfig};

use crate::configs::git_dirty::GitDirtyConfig;
use crate::formatter::StringFormatter;

/// Creates a module with the Git branch in the current directory
///
/// Will display the branch name if the current directory is a git repo
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("git_dirty");
    let config = GitDirtyConfig::try_load(module.config);

    let mut opts = StatusOptions::new();
    opts.include_ignored(false)
        .include_untracked(true)
        .recurse_untracked_dirs(false)
        .exclude_submodules(false);

    let repo = context.get_repo().ok()?;
    let git_repo = Repository::open(repo.path.as_path()).ok()?;

    let git_statuses = git_repo.statuses(Some(&mut opts)).ok()?;
    let is_clean = git_statuses.is_empty();

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|var, _| match var {
                "symbol" => {
                    if is_clean {
                        None
                    } else {
                        Some(config.symbol)
                    }
                }
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `git_dirty`: \n{}", error);
            return None;
        }
    });

    Some(module)
}