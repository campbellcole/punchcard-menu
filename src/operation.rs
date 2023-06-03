use crate::prelude::*;

#[derive(Debug)]
pub enum Operation {
    Clock(clock::ClockOperation),
    Status(status::StatusOperation),
    Project(project::ProjectOperation),
    Report(report::ReportOperation),
    Exit,
    #[cfg(feature = "generate-data")]
    GenerateData(generate::GenerateDataOperation),
}
