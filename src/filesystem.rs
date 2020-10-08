use std::fs::{remove_dir_all, remove_file, File};
use std::io;
use std::io::Read;
use std::path::PathBuf;
use symlink::{remove_symlink_file, symlink_dir, symlink_file};

#[derive(thiserror::Error, Debug)]
pub(crate) enum Error {
    #[error("An invalid path '{0}' is provided.")]
    InvalidPath(String),
    #[error("An IO error occurred.")]
    Io(#[from] io::Error),
}

pub(crate) fn open(path: &PathBuf) -> String {
    let mut file = File::open(path).expect("Failed to open the file.");
    let mut buffer = String::new();

    file.read_to_string(&mut buffer)
        .expect("Failed to read from the file.");

    buffer
}

pub(crate) fn delete(path: &PathBuf) -> Result<(), Error> {
    match (path.is_file(), path.is_dir()) {
        (true, false) => remove_file(path).map_err(Error::Io),
        (false, true) => remove_dir_all(path).map_err(Error::Io),
        _ => Err(Error::InvalidPath(path.to_str().unwrap().to_string())),
    }
}

pub(crate) fn symlink(src: &PathBuf, dest: &PathBuf) -> Result<(), Error> {
    match (src.is_file(), src.is_dir()) {
        (true, false) => symlink_file(src, dest).map_err(Error::Io),
        (false, true) => symlink_dir(src, dest).map_err(Error::Io),
        _ => Err(Error::InvalidPath(src.to_str().unwrap().to_string())),
    }
}

pub(crate) fn symlink_delete(link: &PathBuf) -> Result<(), Error> {
    remove_symlink_file(link).map_err(Error::Io)
}

pub(crate) fn symlink_exists(link: &PathBuf) -> bool {
    link.read_link().is_ok()
}
