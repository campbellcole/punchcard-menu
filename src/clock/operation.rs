use crate::prelude::*;

#[derive(Debug)]
pub struct ClockOperation {
    pub clock_type: ClockType,
    pub project: String,
    pub offset: Option<BiDuration>,
}

#[derive(Debug, Clone, Copy, Default, EnumIter)]
pub enum ClockType {
    In,
    Out,
    #[default]
    Toggle,
}

impl ToString for ClockType {
    fn to_string(&self) -> String {
        match self {
            Self::In => "Clock in",
            Self::Out => "Clock out",
            Self::Toggle => "Toggle clock",
        }
        .into()
    }
}
