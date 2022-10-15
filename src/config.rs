use super::errors::FxError;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;

use crate::state::FX_CONFIG_DIR;

const CONFIG_FILE: &str = "config.toml";

pub const CONFIG_EXAMPLE: &str = "
# Default exec command when open files.
default = \"nvim\"

# (Optional) Whether to use the full width of terminal.
# If not set, this will be true.
# use_full_width = true

# (Optional) Set the max length of item name to be displayed.
# This works only when use_full_width is set to false.
# If the terminal size is not enough, the length will be changed to fit it.
# If not set, this will be 30.
# item_name_length = 30

# (Optional)
# key (the command you want to use) = [values] (extensions)
# [exec]
# feh = [\"jpg\", \"jpeg\", \"png\", \"gif\", \"svg\"]
# zathura = [\"pdf\"]

# The foreground color of directory, file and symlink.
# Pick one of the following:
#   AnsiValue(u8)
#   Black
#   Blue
#   Cyan
#   Green
#   LightBlack
#   LightBlue
#   LightCyan
#   LightGreen
#   LightMagenta
#   LightRed
#   LightWhite
#   LightYellow
#   Magenta
#   Red
#   Rgb(u8, u8, u8)
#   White
#   Yellow
# For more details, see https://docs.rs/termion/1.5.6/termion/color/index.html
[color]
dir_fg = \"LightCyan\"
file_fg = \"LightWhite\"
symlink_fg = \"LightYellow\"
";

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub default: String,
    pub exec: Option<HashMap<String, Vec<String>>>,
    pub color: Color,
    pub use_full_width: Option<bool>,
    pub item_name_length: Option<usize>,
    pub command_mode_bottom: Option<bool>
}

#[derive(Deserialize, Debug, Clone)]
pub struct Color {
    pub dir_fg: Colorname,
    pub file_fg: Colorname,
    pub symlink_fg: Colorname,
}

#[derive(Deserialize, Debug, Clone)]
pub enum Colorname {
    AnsiValue(u8),
    Black,
    Blue,
    Cyan,
    Green,
    LightBlack,
    LightBlue,
    LightCyan,
    LightGreen,
    LightMagenta,
    LightRed,
    LightWhite,
    LightYellow,
    Magenta,
    Red,
    Rgb(u8, u8, u8),
    White,
    Yellow,
}

pub fn read_config() -> Result<Config, FxError> {
    let mut config = dirs::config_dir().unwrap_or_else(|| panic!("Cannot read config dir."));
    config.push(FX_CONFIG_DIR);
    config.push(CONFIG_FILE);
    let config = read_to_string(config.as_path());
    if let Ok(config) = config {
        let deserialized: Config = toml::from_str(&config)?;
        Ok(deserialized)
    } else {
        panic!("Cannot deserialize config file.");
    }
}

pub fn make_config_if_not_exist(config_file: &Path, trash_dir: &Path) -> Result<(), FxError> {
    if !trash_dir.exists() {
        std::fs::create_dir_all(trash_dir)?;
    }

    if !config_file.exists() {
        println!(
            "Config file not found: To set up, Please enter the default command to open a file. (e.g. nvim)"
        );

        let mut buffer = String::new();
        let stdin = std::io::stdin();
        stdin.read_line(&mut buffer)?;

        let trimmed = buffer.trim();
        let config = CONFIG_EXAMPLE.replace("nvim", trimmed);
        std::fs::write(&config_file, config)
            .unwrap_or_else(|_| panic!("cannot write new config file."));
        if cfg!(target_os = "mac_os") {
            println!(
                "Default command set as [{}].\nSee ~/Library/Application Support/felix/config.toml",
                trimmed
            );
        } else {
            println!(
                "Default command set as [{}].\nSee ~/.config/felix/config.toml",
                trimmed
            );
        }
    }

    Ok(())
}
