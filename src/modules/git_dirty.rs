use super::{Context, Module, ModuleConfig};

use crate::configs::git_dirty::GitDirtyConfig;
use crate::formatter::StringFormatter;

use std::ffi::OsStr;

/// Creates a module with the Git branch in the current directory
///
/// Will display the branch name if the current directory is a git repo
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("git_dirty");
    let config = GitDirtyConfig::try_load(module.config);

    context.get_repo().ok()?;

    let is_clean = is_repo_clean(context, &config.clone())?;

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

fn is_repo_clean(context: &Context, _config: &GitDirtyConfig) -> Option<bool> {
    log::debug!("New repo status created");

    let args = vec![
        OsStr::new("-C"),
        context.current_dir.as_os_str(),
        OsStr::new("--no-optional-locks"),
        OsStr::new("status"),
        OsStr::new("--porcelain"),
        OsStr::new("--untracked-files=no"),
    ];

    let status_output = context.exec_cmd("git", &args)?;
    // let statuses = status_output.stdout.lines();

    Some(status_output.stdout.trim().is_empty())
}
