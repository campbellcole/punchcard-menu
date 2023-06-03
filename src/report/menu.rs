use crate::prelude::*;

pub struct ReportMenu;

impl Menu for ReportMenu {
    type Operation = report::ReportOperation;

    fn prompt(_theme: &dyn Theme) -> Result<Self::Operation> {
        todo!()
    }
}
