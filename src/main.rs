use color_eyre::Result;
use dialoguer::theme::ColorfulTheme;
use menu::MainMenu;
use prelude::{Menu, Operation};
use tracing_error::ErrorLayer;
use tracing_subscriber::{prelude::*, EnvFilter};

#[macro_use]
extern crate tracing;
#[macro_use]
extern crate serde;

pub mod data;
pub mod menu;
pub mod operation;
pub mod promptable;
pub mod types;

pub mod clock;
#[cfg(feature = "generate-data")]
pub mod generate;
pub mod prelude;
pub mod project;
pub mod report;
pub mod status;

fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(EnvFilter::from_default_env())
        .with(ErrorLayer::default())
        .init();

    color_eyre::install()?;

    let theme = ColorfulTheme::default();

    loop {
        let operation = match MainMenu::prompt(&theme) {
            Ok(op) => op,
            Err(err) => {
                error!("{}", err);
                continue;
            }
        };

        println!("The menu produced the following operation:");
        println!("{:?}", operation);
        println!("Happily doing absolutely nothing with it :)");

        if matches!(operation, Operation::Exit) {
            break;
        }
    }

    Ok(())
}
