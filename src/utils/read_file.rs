use std::{fs, path::PathBuf};

use crate::types::goal::Goal;

pub fn read_goals(path_to_config: PathBuf) -> Vec<Goal> {
    let data = fs::read_to_string(path_to_config).expect("Could not read goal file");
    serde_yml::from_str(&data).expect("Could not convert yml to data structure")
}
