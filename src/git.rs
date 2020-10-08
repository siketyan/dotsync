use std::io;
use std::path::PathBuf;
use std::process::{Command, Output};

#[derive(thiserror::Error, Debug)]
pub(crate) enum Error {
    #[error("The provided path was not a valid UTF-8 string.")]
    NonUtf8String,

    #[error("Failed to create a process.")]
    CreatingProcess(#[from] io::Error),

    #[error("The process exited with non-zero code, stdout was '{0}' and stderr was '{1}'.")]
    ExitedWithNonZero(String, String),
}

pub(crate) struct Repository {
    path: PathBuf,
}

impl Repository {
    pub(crate) fn from(path: &PathBuf) -> Self {
        Self { path: path.clone() }
    }

    pub(crate) fn clone(source: &str, destination: &PathBuf) -> Result<Self, Error> {
        let output = run_git(&[
            "clone",
            source,
            destination.to_str().ok_or(Error::NonUtf8String)?,
        ])?;
        if output.status.success() {
            Ok(Self::from(destination))
        } else {
            Err(Error::ExitedWithNonZero(
                String::from_utf8(output.stdout.into()).map_err(|_| Error::NonUtf8String)?,
                String::from_utf8(output.stderr.into()).map_err(|_| Error::NonUtf8String)?,
            ))
        }
    }

    pub(crate) fn pull(&self) -> Result<(), Error> {
        run_git_with_pwd(&["pull"], &self.path).map(|_| ())
    }
}

fn run_git(args: &[&str]) -> Result<Output, Error> {
    Command::new("git")
        .args(args)
        .output()
        .map_err(|err| Error::CreatingProcess(err))
}

fn run_git_with_pwd(args: &[&str], pwd: &PathBuf) -> Result<Output, Error> {
    Command::new("git")
        .args(args)
        .current_dir(pwd)
        .output()
        .map_err(|err| Error::CreatingProcess(err))
}
