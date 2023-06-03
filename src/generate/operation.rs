use crate::prelude::*;

#[derive(Debug)]
pub struct GenerateDataOperation {
    pub count: usize,
    pub output_file: Option<PathBuf>,
}
