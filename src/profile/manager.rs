use clap::error::Result;
use clap::Error;

use crate::profile::git::GitProfile;
use crate::Profile;

pub trait ProfileManager {
    fn new() -> Self;
    fn parse_config(&mut self, config: &str) -> Result<(), Box<dyn std::error::Error>>;
    fn get_profiles(&self, profile_type: &str) -> Vec<&Box<dyn Profile>>;
    fn get_profiles_with_title(&self, profile_type: &str, title: &str) -> Vec<&Box<dyn Profile>>;
    fn get_profile_types(&self) -> Vec<String>;
    fn get_all_profiles(&self) -> Vec<&Box<dyn Profile>>;
    fn add_profile(&mut self, service: Box<dyn Profile>);
}

pub struct ProfileManagerDefault {
    profiles: Vec<Box<dyn Profile>>,
}

impl ProfileManager for ProfileManagerDefault {
    fn new() -> Self {
        return ProfileManagerDefault {
            profiles: Vec::new(),
        };
    }
    fn get_profiles(&self, profile_type: &str) -> Vec<&Box<dyn Profile>> {
        return self
            .profiles
            .iter()
            .filter(|x| x.profile_type() == profile_type)
            .collect();
    }
    fn get_profiles_with_title(&self, profile_type: &str, title: &str) -> Vec<&Box<dyn Profile>> {
        return self
            .profiles
            .iter()
            .filter(|x| x.profile_type() == profile_type && x.title().unwrap() == title)
            .collect();
    }

    fn get_profile_types(&self) -> Vec<String> {
        return Vec::new();
    }
    fn get_all_profiles(&self) -> Vec<&Box<dyn Profile>> {
        return self.profiles.iter().collect();
    }
    fn add_profile(&mut self, service: Box<dyn Profile>) {
        self.profiles.push(service);
    }
    fn parse_config(&mut self, config: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("Parsing config from: {}", config);
        let content = std::fs::read_to_string(config)?;

        let mut last_profile: Option<Box<dyn Profile>> = None;

        for line in content.lines() {
            if line.starts_with("#") || line.is_empty() {
                continue;
            } else if line.starts_with("[") && line.ends_with("]") {
                if (last_profile.is_some()) {
                    self.add_profile(last_profile.as_mut().unwrap().copy());
                }

                let title = line.trim_start_matches("[").trim_end_matches("]");
                last_profile = Some(instantiate_profile(title)?);
            } else if line.contains("=") && last_profile.is_some() {
                let mut split = line.split("=");
                let key = split.next().unwrap();
                let value = split.next().unwrap();

                last_profile
                    .as_mut()
                    .unwrap()
                    .config_data_mut()
                    .insert(key.to_string(), value.to_string());
            } else {
                return Err("Config file is not formatted correctly".into());
            }
        }
        if (last_profile.is_some()) {
            self.add_profile(last_profile.as_mut().unwrap().copy());
        }

        return Ok(());
    }
}

fn instantiate_profile(title: &str) -> Result<Box<dyn Profile>, Box<dyn std::error::Error>> {
    let mut profile: Box<dyn Profile>;

    match title {
        "git" => {
            profile = Box::new(GitProfile::default());
        }
        _ => {
            return Err("Profile type not recognized".into());
        }
    }

    return Ok(profile);
}
