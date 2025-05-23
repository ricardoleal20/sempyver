// Module declarations
pub mod ai_calls;
pub mod ai_message_generator;
pub mod changelog_utils;
pub mod changeset_structures;
pub mod version_operations;

// Re-exports
pub use ai_message_generator::{generate_ai_message, AIConfig};
pub use changelog_utils::{create_changelog, new_changelog_entry, open_changelog};

/// Make the modules accessible
mod changesets_utilities;
mod sets_utils;
mod subcommands;
// Local imports
use crate::options::Changeset;
pub use changesets_utilities::get_current_changesets;
pub use sets_utils::{create_changeset_folder, write_changeset_file};
pub use subcommands::create_subcommands;
// Libraries to use
use regex::Regex;
use std::fs;
use std::io::{BufRead, BufReader, Read, Write};
use toml::Value;

pub fn find_version() -> String {
    // Find the version in the current path
    let version_paths = find_version_in_file();
    // Using this, return the version
    open_path(version_paths[0].clone())
}

pub fn find_version_in_file() -> Vec<String> {
    // Search the `pyproject.toml` in the root folder
    let route = "pyproject.toml";

    // Read the pyproject.toml content
    let config = match fs::read_to_string(route) {
        Ok(config) => config,
        Err(e) => {
            panic!("Error reading the `pyproject.toml` file: {}", e);
        }
    };

    // Parse the content as a TOML file
    let toml_config: Value = match config.parse() {
        Ok(toml_config) => toml_config,
        Err(e) => {
            panic!("Error getting the file {}: {}", route, e)
        }
    };

    // Search the [tool.versionwise] version path
    let mut version_paths: Vec<String> = Vec::new();
    if let Some(tool) = toml_config.get("tool") {
        if let Some(versionwise) = tool.get("versionwise") {
            if let Some(possible_paths) = versionwise.get("version_path") {
                if let Some(paths) = possible_paths.as_array() {
                    for path in paths {
                        version_paths.push(path.to_string().replace("\"", ""));
                    }
                } else {
                    panic!("The version path doesn't include a path");
                }
            } else {
                panic!("The versionwise utility doesn't include a `version_path` field")
            }
        } else {
            panic!(
                "The pyproject doesn't have a versionwise as tool. You should have [tool.versionwise]."
            )
        }
    } else {
        panic!("The pyproject doesn't have tools associated. Please add the `versionwise` tool as [tool.versionwise].")
    }
    if version_paths.is_empty() {
        panic!("Couldn't find any version paths in the configuration.")
    }
    // Return the version paths
    version_paths
}

pub fn open_path(path: String) -> String {
    // Open the file
    let file = match fs::File::open(path.clone()) {
        Ok(file) => file,
        Err(e) => {
            panic!("Error opening file {}: {}.", path, e);
        }
    };
    // Create the buffer to read the file
    let reader = BufReader::new(file);
    // Iterate over the lines in the file to get the version
    for line in reader.lines() {
        if let Ok(line) = line {
            // Verify if the line has the pattern
            if line.contains("version =") || line.contains("__version__ =") {
                // Initialize the process extraction
                let pattern = r#""(\d+\.\d+\.\d+)""#;
                // Compilar el patrón de expresión regular
                let re = Regex::new(pattern).unwrap();
                if let Some(captures) = re.captures(&line) {
                    if let Some(version) = captures.get(1) {
                        return version.as_str().to_string();
                    }
                } else {
                    panic!(
                        "In the line \"{}\" it cannot be found a version number.",
                        line
                    );
                }
            }
        } else {
            panic!("Error reading the file {}.", path);
        }
    }
    // If it reaches here, then it couldn't find the `version`
    panic!("Couldn't find the version in the path {}. Try with the following version names: [\"version\", \"__version__\"]", path);
}

fn update_version_path(new_version: &str) {
    // Find all version paths
    let version_paths = find_version_in_file();
    // Get the current version
    let current_version = find_version();

    // Update each file
    for version_path in version_paths {
        // Open the file
        let mut file = match fs::File::open(&version_path) {
            Ok(file) => file,
            Err(e) => {
                panic!("Error opening file {}: {}.", version_path, e);
            }
        };
        // Read the content as a String
        let mut content = String::new();
        if let Err(e) = file.read_to_string(&mut content) {
            panic!("Error reading file {}: {}.", version_path, e);
        }
        // Substitute the old version for the new version
        let updated_content = content.replace(&current_version, new_version);
        // Reopen the file but this time as writing mode
        file = match fs::File::create(&version_path) {
            Ok(file) => file,
            Err(e) => {
                panic!("Error creating file {}: {}.", version_path, e);
            }
        };
        // Write the new file
        if let Err(e) = file.write_all(updated_content.as_bytes()) {
            panic!("Error writing to file {}: {}.", version_path, e);
        }
    }
}

/// Find the largest version in a list of changesets
pub fn find_largest_version(changesets: &[Changeset]) -> Option<String> {
    changesets
        .iter()
        .filter_map(|c| parse_version(&c.version)) // Parse the versions
        .max() // Obtain the largest version
        .map(|(major, minor, patch)| format!("{}.{}.{}", major, minor, patch)) // Convert it back to String
}

/// Parse a version "MAJOR.MINOR.PATCH" into a tuple (u32, u32, u32)
fn parse_version(version: &str) -> Option<(u32, u32, u32)> {
    let parts: Vec<u32> = version
        .split('.') // Divide into parts
        .filter_map(|p| p.parse().ok()) // Convert to u32
        .collect();

    if parts.len() == 3 {
        Some((parts[0], parts[1], parts[2]))
    } else {
        Some((0, 0, 0))
    }
}
