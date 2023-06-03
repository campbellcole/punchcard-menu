use crate::prelude::*;

pub mod projects;

pub fn data_dir() -> Result<PathBuf> {
    dirs::data_dir().or_else(|| std::env::var("PUNCHCARD_DATA_FOLDER").ok().map(PathBuf::from))
        .ok_or(eyre!("Could not find data directory. Please set the PUNCHCARD_DATA_FOLDER environment variable."))
        .map(|p| p.join("punchcard2"))
}
