use std::sync::Mutex;

use lazy_static::lazy_static;

use super::data_dir;

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Projects(Vec<String>);

lazy_static! {
    pub static ref PROJECTS: Mutex<Projects> = Mutex::new(Projects::load());
}

impl AsRef<Vec<String>> for Projects {
    fn as_ref(&self) -> &Vec<String> {
        &self.0
    }
}

impl Projects {
    pub fn load() -> Self {
        let projects_path = data_dir().unwrap().join("projects.json");

        if projects_path.exists() {
            let file = std::fs::File::open(projects_path).unwrap();
            serde_json::from_reader(file).unwrap()
        } else {
            Self(Vec::new())
        }
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, idx: usize) -> Option<&String> {
        self.0.get(idx)
    }

    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.0.iter()
    }

    pub fn add(&mut self, project: String) {
        self.0.push(project);
        self.save();
    }

    pub fn remove(&mut self, project: &str) {
        self.0.retain(|p| p != project);
        self.save();
    }

    pub fn save(&self) {
        let projects_path = data_dir().unwrap().join("projects.json");
        let file = std::fs::File::create(projects_path).unwrap();
        serde_json::to_writer_pretty(file, self).unwrap();
    }
}
