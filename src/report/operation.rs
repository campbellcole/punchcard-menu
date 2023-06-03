use crate::prelude::*;

#[derive(Debug)]
pub struct ReportOperation {
    pub report_type: ReportType,
    pub output_file: Option<PathBuf>,
    pub exact_durations: bool,
    pub copyable: bool,
}

#[derive(Debug, Default)]
pub enum ReportType {
    Daily,
    #[default]
    Weekly,
    // Monthly,
    // Yearly,
}
