use std::path::{Path, PathBuf};

pub fn global_options_data_path(config_dir: &Path) -> PathBuf {
    config_dir.join("global-options.json")
}

pub fn avatar_data_path(config_dir: &Path) -> PathBuf {
    config_dir.join("avatars.json")
}

pub fn options_data_path(config_dir: &Path) -> PathBuf {
    config_dir.join("profile-options.json")
}
