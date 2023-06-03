use crate::{data::projects::PROJECTS, prelude::*};

use super::ProjectOperationType;
pub struct ProjectMenu;

impl Menu for ProjectMenu {
    type Operation = project::ProjectOperation;

    fn prompt(theme: &dyn Theme) -> Result<Self::Operation> {
        let items = ProjectOperationType::iter().collect::<Vec<_>>();
        let selection = FuzzySelect::with_theme(theme)
            .with_prompt("Select project operation:")
            .items(&items)
            .interact()?;

        let project = match &items[selection] {
            ProjectOperationType::Add => Input::<String>::with_theme(theme)
                .with_prompt("Enter a name for the project:")
                .interact_text()?,
            _ => {
                let projects = PROJECTS.lock().unwrap();

                if projects.len() == 0 {
                    return Err(eyre!("No projects exist. Please add one first."));
                }

                let selection = FuzzySelect::with_theme(theme)
                    .items(projects.as_ref())
                    .with_prompt("Select a project:")
                    .interact()?;

                projects.get(selection).unwrap().clone()
            }
        };

        let op_type = match &items[selection] {
            ProjectOperationType::Rename(_) => {
                let new_name = Input::<String>::with_theme(theme)
                    .with_prompt("Enter a new name:")
                    .interact_text()?;
                ProjectOperationType::Rename(new_name)
            }
            op => op.clone(),
        };

        Ok(Self::Operation { op_type, project })
    }
}
