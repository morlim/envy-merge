

use std::collections::HashMap;
use std::path::Path;
use dotenvy::from_path_iter;
use anyhow::Result;

/// Reads and merges multiple `.env` files while ensuring:
/// - The priority file (if provided) **takes absolute precedence**.
/// - Later `.env` files overwrite earlier ones **unless the key exists in the priority file**.
pub fn merge_env_files(files: &[&str], priority_file: Option<&String>) -> Result<String> {
    let mut env_map: HashMap<String, String> = HashMap::new();
    let mut priority_vars = HashMap::new();

    // Load priority file first (if provided)
    if let Some(priority) = priority_file {
        if let Ok(vars) = read_env_file(priority) {
            priority_vars = vars.clone(); // Store priority variables separately
            env_map.extend(vars); // Add priority variables to the main map
        }
    }

    // Process regular `.env` files, allowing overwrites **only if not in priority_vars**
    for file in files {
        if let Ok(vars) = read_env_file(file) {
            for (key, value) in vars {
                if !priority_vars.contains_key(&key) {
                    env_map.insert(key, value); // Overwrite only if not in priority file
                }
            }
        }
    }

    // Convert to .env formatted string
    let merged_env = env_map
        .into_iter()
        .map(|(key, value)| format!("{}={}", key, value))
        .collect::<Vec<String>>()
        .join("\n");

    Ok(merged_env)
}

pub fn read_env_file(file: &str) -> anyhow::Result<HashMap<String, String>> {
    let path = Path::new(file);
    let mut vars = HashMap::new();

    if path.exists() {
        for item in from_path_iter(path)? {
            let (key, value) = item?;
            vars.insert(key, value);
        }
    }

    Ok(vars)
}