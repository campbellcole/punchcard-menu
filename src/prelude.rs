pub use crate::{menu::Menu, operation::*, promptable::Promptable, types::*};

#[cfg(feature = "generate-data")]
pub use crate::generate;
pub use crate::{clock, project, report, status};

pub use color_eyre::{
    eyre::{eyre, Context},
    Result,
};
pub use dialoguer::{
    theme::ColorfulTheme, theme::Theme, Confirm, Editor, FuzzySelect, Input, MultiSelect, Password,
    Select, Sort,
};
pub use strum::{EnumIter, IntoEnumIterator};

pub use std::{io, path::PathBuf};

macro_rules! operation_module {
    () => {
        mod menu;
        mod operation;
        pub use menu::*;
        pub use operation::*;
    };
}

pub(crate) use operation_module;
