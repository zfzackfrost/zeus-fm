use directories::ProjectDirs;
use std::path::{PathBuf};

lazy_static! {
    pub static ref CONFIG_DIR: Option<PathBuf> = {
        if let Some(proj_dirs) = ProjectDirs::from("com", "",  "ZeusFm") {
            Some(proj_dirs.config_dir().to_path_buf())
        } else {
            None
        }
    };
}
