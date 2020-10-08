mod filesystem;
mod git;
mod mapping;

use dirs::home_dir;
use std::env::args;
use std::path::PathBuf;

use crate::filesystem::*;
use crate::git::Repository;
use crate::mapping::Mapping;

#[derive(thiserror::Error, Debug)]
pub(crate) enum Error {
    #[error("A filesystem error occurred.")]
    Filesystem(#[from] filesystem::Error),

    #[error("A Git error occurred.")]
    Git(#[from] git::Error),

    #[error("A mapping error occurred.")]
    Mapping(#[from] mapping::Error),

    #[error("A home directory cannot be found.")]
    HomeDirectoryNotFound,

    #[error("The arguments you specified are not enough.")]
    NotEnoughArguments,

    #[error("The file '{0}' does not exist.")]
    NotExists(PathBuf),
}

fn main() -> Result<(), Error> {
    let home_dir = home_dir().ok_or(Error::HomeDirectoryNotFound)?;
    let directory = home_dir.join(".dotsync");

    if directory.exists() {
        Repository::from(&directory).pull()
    } else {
        let url = args().nth(1).ok_or(Error::NotEnoughArguments)?;
        Repository::clone(&url, &directory).map(|_| ())
    }
    .map_err(Error::Git)?;

    let file = directory.join(".dotsyncrc");
    let mapping = Mapping::from_yaml_file(&file).map_err(Error::Mapping)?;

    mapping
        .expand(&directory)
        .iter()
        .map(|(src, dest)| {
            if symlink_exists(dest) {
                symlink_delete(dest).map_err(Error::Filesystem)?
            } else if dest.exists() {
                delete(dest).map_err(Error::Filesystem)?;
            }

            if src.exists() {
                symlink(src, dest).map_err(Error::Filesystem)
            } else {
                Err(Error::NotExists(src.clone()))
            }
        })
        .collect()
}
