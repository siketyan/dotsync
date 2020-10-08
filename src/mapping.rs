use std::collections::HashMap;
use std::path::PathBuf;

use crate::filesystem;
use std::ops::Deref;

#[derive(thiserror::Error, Debug)]
pub(crate) enum Error {
    #[error("Failed to parse the YAML string.")]
    ParsingYaml(#[from] serde_yaml::Error),
}

pub(crate) struct Mapping {
    paths: HashMap<String, String>,
}

impl Mapping {
    pub(crate) fn from_yaml(yaml: &str) -> Result<Self, Error> {
        Ok(Self {
            paths: serde_yaml::from_str(yaml).map_err(Error::ParsingYaml)?,
        })
    }

    pub(crate) fn from_yaml_file(path: &PathBuf) -> Result<Self, Error> {
        Self::from_yaml(&filesystem::open(path))
    }

    pub(crate) fn expand(&self, directory: &PathBuf) -> HashMap<PathBuf, PathBuf> {
        self.paths
            .iter()
            .map(|(src, dest)| {
                (
                    directory.join(src),
                    PathBuf::from(shellexpand::tilde(dest).deref()),
                )
            })
            .collect()
    }
}
