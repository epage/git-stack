use std::io::Write;

use proc_exit::WithCodeResultExt;

pub fn dump_config(
    args: &crate::args::Args,
    output_path: &std::path::Path,
) -> proc_exit::ExitResult {
    log::trace!("Initializing");
    let cwd = std::env::current_dir().with_code(proc_exit::Code::USAGE_ERR)?;
    let repo = git2::Repository::discover(&cwd).with_code(proc_exit::Code::USAGE_ERR)?;

    let repo_config = git_stack::config::RepoConfig::from_all(&repo)
        .with_code(proc_exit::Code::CONFIG_ERR)?
        .update(args.to_config());

    let output = repo_config.to_string();

    if output_path == std::path::Path::new("-") {
        std::io::stdout().write_all(output.as_bytes())?;
    } else {
        std::fs::write(output_path, &output)?;
    }

    Ok(())
}

pub fn protect(args: &crate::args::Args, ignore: &str) -> proc_exit::ExitResult {
    log::trace!("Initializing");
    let cwd = std::env::current_dir().with_code(proc_exit::Code::USAGE_ERR)?;
    let repo = git2::Repository::discover(&cwd).with_code(proc_exit::Code::USAGE_ERR)?;

    let mut repo_config = git_stack::config::RepoConfig::from_repo(&repo)
        .with_code(proc_exit::Code::CONFIG_ERR)?
        .update(args.to_config());
    repo_config
        .protected_branches
        .get_or_insert_with(Vec::new)
        .push(ignore.to_owned());

    repo_config
        .write_repo(&repo)
        .with_code(proc_exit::Code::FAILURE)?;

    Ok(())
}

pub fn protected(args: &crate::args::Args) -> proc_exit::ExitResult {
    log::trace!("Initializing");
    let cwd = std::env::current_dir().with_code(proc_exit::Code::USAGE_ERR)?;
    let repo = git2::Repository::discover(&cwd).with_code(proc_exit::Code::USAGE_ERR)?;

    let repo_config = git_stack::config::RepoConfig::from_all(&repo)
        .with_code(proc_exit::Code::CONFIG_ERR)?
        .update(args.to_config());
    let protected = git_stack::git::ProtectedBranches::new(
        repo_config.protected_branches().iter().map(|s| s.as_str()),
    )
    .with_code(proc_exit::Code::CONFIG_ERR)?;

    let repo = git_stack::git::GitRepo::new(repo);
    let branches = git_stack::graph::BranchSet::from_repo(&repo, &protected)
        .with_code(proc_exit::Code::FAILURE)?;

    for (_, branches) in branches.iter() {
        for branch in branches {
            match branch.kind() {
                git_stack::graph::BranchKind::Deleted
                | git_stack::graph::BranchKind::Mutable
                | git_stack::graph::BranchKind::Mixed => {
                    log::debug!("{:?}: {}", branch.kind(), branch.display_name());
                }
                git_stack::graph::BranchKind::Protected => {
                    log::debug!("{:?}: {}", branch.kind(), branch.display_name());
                    writeln!(std::io::stdout(), "{}", branch.display_name())?;
                }
            }
        }
    }

    Ok(())
}
