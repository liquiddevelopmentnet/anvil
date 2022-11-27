use serde::{Serialize};
use serde::Deserialize;
use crate::*;
use crate::config::{save_config, YamlConfig};

static mut CONFIG: Option<ProjectsConfiguration> = None;

impl YamlConfig for ProjectsConfiguration {
    const DEFAULT_CONFIG_NAME: &'static str = "anvil-projects.yml";

    fn new() -> Self {
        ProjectsConfiguration {
            projects: vec![
                Project {
                    id: "your-project".to_string(),
                    friendly_name: "Your Project".to_string(),
                    git_url: "https://github.com/username/project".to_string(),
                    channels: vec![
                        Channel {
                            id: "production".to_string(),
                            active: true,
                            level: 0,
                            public: true,
                        },
                        Channel {
                            id: "canary".to_string(),
                            active: true,
                            level: 1,
                            public: true,
                        },
                        Channel {
                            id: "dev".to_string(),
                            active: true,
                            level: 2,
                            public: false,
                        }
                    ],
                }
            ]
        }
    }

    fn save(&self) -> Result<(), io::Error> {
        save_config(self, Self::DEFAULT_CONFIG_NAME)
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ProjectsConfiguration {
    pub projects: Vec<Project>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Project {
    pub id: String,
    pub friendly_name: String,
    pub git_url: String,
    pub channels: Vec<Channel>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Channel {
    pub id: String,
    pub active: bool,
    pub level: u8,
    pub public: bool,
}

pub fn load() -> bool {
    unsafe {
        let load = config::load_config(ProjectsConfiguration::DEFAULT_CONFIG_NAME);
        if load.is_some() {
            CONFIG = load;
            false
        } else {
            true
        }
    }
}

pub fn get() -> ProjectsConfiguration {
    unsafe {
        CONFIG.as_ref().unwrap().clone()
    }
}