use crate::prelude::*;

pub struct StatusMenu;

impl Menu for StatusMenu {
    type Operation = status::StatusOperation;

    fn prompt(theme: &dyn Theme) -> Result<Self::Operation> {
        let offset = Option::<BiDuration>::prompt(theme, "Enter an offset:")?;

        Ok(Self::Operation { offset })
    }
}
