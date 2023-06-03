use crate::prelude::*;

pub trait Promptable
where
    Self: Sized,
{
    fn prompt(theme: &dyn Theme, prompt: &str) -> Result<Self>;
}

impl Promptable for BiDuration {
    fn prompt(theme: &dyn Theme, prompt: &str) -> Result<Self> {
        let input = Input::<String>::with_theme(theme)
            .with_prompt(prompt)
            .interact_text()?;

        Ok(input.parse::<BiDuration>()?)
    }
}

impl Promptable for PathBuf {
    fn prompt(theme: &dyn Theme, prompt: &str) -> Result<Self> {
        let input = Input::<String>::with_theme(theme)
            .with_prompt(prompt)
            .interact_text()?;

        Ok(input.into())
    }
}

impl<T: Promptable> Promptable for Option<T> {
    fn prompt(theme: &dyn Theme, prompt: &str) -> Result<Self> {
        let confirmation = Confirm::with_theme(theme)
            .with_prompt(format!(
                "Would you like to {}?",
                prompt.to_lowercase().replace(':', "")
            ))
            .interact()?;

        if confirmation {
            Ok(Some(T::prompt(theme, prompt)?))
        } else {
            Ok(None)
        }
    }
}
