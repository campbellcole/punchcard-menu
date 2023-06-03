use crate::prelude::*;

pub trait Menu {
    type Operation;
    fn prompt(theme: &dyn Theme) -> Result<Self::Operation>;
}

pub struct MainMenu;

impl Menu for MainMenu {
    type Operation = Operation;

    fn prompt(theme: &dyn Theme) -> Result<Self::Operation> {
        let items = MainMenuOption::iter().collect::<Vec<_>>();
        let selection = FuzzySelect::with_theme(theme)
            .with_prompt("What would you like to do?")
            .items(&items)
            .interact()?;

        let op = match items[selection] {
            MainMenuOption::Clock => clock::ClockMenu::prompt(theme).map(Operation::Clock)?,
            MainMenuOption::Status => status::StatusMenu::prompt(theme).map(Operation::Status)?,
            MainMenuOption::Project => {
                project::ProjectMenu::prompt(theme).map(Operation::Project)?
            }
            MainMenuOption::Report => report::ReportMenu::prompt(theme).map(Operation::Report)?,
            MainMenuOption::Exit => Operation::Exit,
            #[cfg(feature = "generate-data")]
            MainMenuOption::GenerateData => {
                generate::GenerateDataMenu::prompt(theme).map(Operation::GenerateData)?
            }
        };

        Ok(op)
    }
}

#[derive(Debug, EnumIter)]
enum MainMenuOption {
    Clock,
    Status,
    Project,
    Report,
    #[cfg(feature = "generate-data")]
    GenerateData,
    // last so it appears last on the menu
    Exit,
}

impl ToString for MainMenuOption {
    fn to_string(&self) -> String {
        match self {
            Self::Clock => "Add clock entry",
            Self::Status => "View current status",
            Self::Project => "Manage projects",
            Self::Report => "Generate a report",
            Self::Exit => "Exit",
            #[cfg(feature = "generate-data")]
            Self::GenerateData => "Generate data",
        }
        .into()
    }
}
