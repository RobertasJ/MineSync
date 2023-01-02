use std::fs;
use toml;
use std::error;

pub fn check_entries(config: &mut toml::value::Table, entries: Vec<(&str, toml::Value)>) {
    for (entry, value) in entries {
        if config.get(entry).is_none() {
            config.insert(entry.to_string(), value.to_owned());
        }
    }
}

pub fn open_config(filepath: &str) -> Result<toml::value::Table, Box<dyn error::Error>> {
    let contents = fs::read_to_string(filepath)?;
    let config = toml::from_str(&contents)?;
    Ok(config)
}

pub fn write_config(config: &toml::value::Table, filepath: &str) -> Result<(), Box<dyn error::Error>> {
    let output_string = toml::to_string(config)?;
    fs::write(filepath, output_string)?;
    Ok(())
}
