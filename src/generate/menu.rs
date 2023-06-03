use crate::prelude::*;

pub struct GenerateDataMenu;

impl Menu for GenerateDataMenu {
    type Operation = generate::GenerateDataOperation;

    fn prompt(theme: &dyn Theme) -> Result<Self::Operation> {
        let count = Input::<usize>::with_theme(theme)
            .with_prompt("How many entries would you like to generate?")
            .interact()?;

        let output_file = Option::<PathBuf>::prompt(theme, "Enter an output file:")?;

        Ok(Self::Operation { count, output_file })
    }
}
