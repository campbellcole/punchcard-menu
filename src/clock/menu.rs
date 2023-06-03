use crate::{data::projects::PROJECTS, prelude::*};

use super::ClockType;

pub struct ClockMenu;

impl Menu for ClockMenu {
    type Operation = clock::ClockOperation;

    fn prompt(theme: &dyn Theme) -> Result<Self::Operation> {
        let projects = PROJECTS.lock().unwrap();

        if projects.len() == 0 {
            return Err(eyre!("No projects exist. Please add one first."));
        }

        let selection = FuzzySelect::with_theme(theme)
            .items(projects.as_ref())
            .with_prompt("Select a project:")
            .interact()?;
        let project = projects.get(selection).unwrap().clone();

        let items = ClockType::iter().collect::<Vec<_>>();
        let selection = FuzzySelect::with_theme(theme)
            .with_prompt("Select clock operation:")
            .items(&items)
            .interact()?;

        let clock_type = items[selection];

        let offset = Option::<BiDuration>::prompt(theme, "Enter an offset:")?;

        Ok(Self::Operation {
            clock_type,
            offset,
            project,
        })
    }
}
