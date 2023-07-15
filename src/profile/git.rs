use clap::{error::Result, Error};

use crate::Profile;
use std::collections::HashMap;
use std::process::Command;

pub struct GitProfile {
    pub config_data: HashMap<String, String>,
    pub email: String,
    pub name: String,
    pub signing_key: String,
    pub config_file: String,
}
impl Profile for GitProfile {
    fn profile_type(&self) -> String {
        return "git".to_string();
    }
    fn config_file(&self) -> String {
        return self.config_file.clone();
    }
    fn config_data(&self) -> &HashMap<String, String> {
        return &self.config_data;
    }
    fn config_data_mut(&mut self) -> &mut HashMap<String, String> {
        return &mut self.config_data;
    }
    fn copy(&self) -> Box<dyn Profile> {
        return Box::new(self.clone());
    }
    fn title(&self) -> Result<String, Box<dyn std::error::Error>> {
        if self.config_data.get("title").is_none() {
            return Err("No title found, configuration malformed.".into());
        }
        return Ok(self.config_data.get("title").unwrap().clone());
    }
    fn apply(&self) -> Result<(), Box<dyn std::error::Error>> {
        set_git_config(
            &"user.name".to_string(),
            self.config_data
                .get("user")
                .expect("No title found, configuration malformed."),
        )?;
        set_git_config(
            &"user.email".to_string(),
            self.config_data
                .get("email")
                .expect("No title found, configuration malformed."),
        )?;

        Ok(())
    }
}
impl Clone for GitProfile {
    fn clone(&self) -> Self {
        return GitProfile {
            config_data: self.config_data.clone(),
            email: self.email.clone(),
            name: self.name.clone(),
            signing_key: self.signing_key.clone(),
            config_file: self.config_file.clone(),
        };
    }
}

impl Default for GitProfile {
    fn default() -> Self {
        return GitProfile {
            config_data: HashMap::new(),
            email: String::new(),
            name: String::new(),
            signing_key: String::new(),
            config_file: String::new(),
        };
    }
}

fn set_git_config(key: &String, value: &String) -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("git")
        .arg("config")
        .arg(key)
        .arg(value)
        .output()
        .expect("failed to execute process");

    assert!(output.status.success());

    Ok(())
}
