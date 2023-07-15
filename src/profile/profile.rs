use std::collections::HashMap;

pub trait Profile {
    //fn new(&self) -> Self;
    fn config_data_mut(&mut self) -> &mut HashMap<String, String>;
    fn config_data(&self) -> &HashMap<String, String>;
    fn profile_type(&self) -> String;
    fn title(&self) -> Result<String, Box<dyn std::error::Error>>;
    fn config_file(&self) -> String;
    fn copy(&self) -> Box<dyn Profile>;
    fn apply(&self) -> Result<(), Box<dyn std::error::Error>>;
}
