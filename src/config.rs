///! Configuration management.
///!
///! Provides structures for the TOML config,
///! an initialization function and types.
use std::{ fs, process::exit, env::var, collections::BTreeMap };
use serde::{ Deserialize, Serialize };
use toml;


/// A single program entry
pub type Entry = Vec<BTreeMap<String, String>>;
/// A binary tree map of multiple program entries
pub type Entries = BTreeMap<String, Entry>;


/// A representation of the app's configuration
/// Loaded via TOML configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// Configuration contains general options and shortcut entries
    pub general: General,
    pub colors: Colors,
    pub shrots: Entries,
}


/// A representation of general options
#[derive(Debug, Serialize, Deserialize)]
pub struct General {
    pub width: i32,         // Window width
    pub height: i32,        // Window height
    pub opacity: f64,       // Window opacity
    pub columns: i32,       // The column amount
    pub border: i32,        // The border in pixels
}


/// A representation of color options
#[derive(Debug, Serialize, Deserialize)]
pub struct Colors {
    pub background: u32,    // The main background color
    pub foreground: u32,    // The main text / foreground color
    pub border: u32,        // The border color
    pub shrot: u32,         // The shortcut color
    pub about: u32,         // The about text color
}


/// Read the configuration file
///
/// Looks for the configuration file in predetermined locations
/// Reads the configuration as a TOML file
/// Returns a `Config` structure
pub fn get_config() -> Config {
    // Config path
    let mut config_path: String;
    
    if fs::metadata("Config.toml").is_ok() {
        // Check if the configuration file is in the current directory
        config_path = "Config.toml".to_string();
    } else if fs::metadata(var("HOME").unwrap() + "/.config/shrots/Config.toml").is_ok() {
        // Check if the configuration file is in the dots directory
        config_path = var("HOME").unwrap().to_owned();
        config_path.push_str("/.config/shrots/Config.toml");
    } else if fs::metadata("/etc/shrots/Config.toml").is_ok() {
        // Check if the configuration file is in the `/etc` directory
        config_path = "/etc/shrots/Config.toml".to_string();
    } else {
        // If not found then exit
        println!("Could not find a valid configuration file. :/");
        exit(1);
    }

    // Read the configuration file
    let text: String = fs::read_to_string(config_path).unwrap();
    return toml::from_str(&text.as_str()).unwrap();
}
