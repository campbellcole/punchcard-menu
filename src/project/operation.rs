use crate::prelude::*;

#[derive(Debug)]
pub struct ProjectOperation {
    pub op_type: ProjectOperationType,
    pub project: String,
}

#[derive(Debug, Clone, EnumIter)]
pub enum ProjectOperationType {
    Add,
    Remove,
    Rename(String),
}

impl ToString for ProjectOperationType {
    fn to_string(&self) -> String {
        match self {
            Self::Add => "Add a project",
            Self::Remove => "Remove a project",
            Self::Rename(_) => "Rename a project",
        }
        .into()
    }
}
