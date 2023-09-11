use anyhow::{Context, Result};
use std::fs;
use yaml_rust::YamlLoader;

pub fn get_option(option_name: &str) -> Result<String> {
    let file = fs::read_to_string("./server.properties")?;
    let options = &YamlLoader::load_from_str(&file)?[0];
    Ok(options[option_name]
        .as_str()
        .context(format!("Failed to get option:{}", option_name))?
        .to_owned())
}
