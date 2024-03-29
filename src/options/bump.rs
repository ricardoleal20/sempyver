/// ================================ ///
///         OPTIONS :: Bump          ///
/// ================================ ///
// Local imports
use crate::utilities::{
    create_changelog, find_version, get_current_changesets, new_changelog_entry, open_changelog,
    update_version,
};

pub fn bump_version() {
    // First, get the changesets
    let changesets = get_current_changesets();
    // Find the current project version
    let new_version = update_version(&changesets, find_version());
    // From here, parse the changesets as the new Changelog entry
    let new_entry = new_changelog_entry(&changesets, &new_version);
    // Now, read the current CHANGESET file
    let mut content = open_changelog();
    // Find the index where it is the start of versions
    let start_of_versions_index = content
        .iter()
        .position(|line| line.starts_with("## ["))
        .unwrap_or(content.len());
    //content_to_write.truncate(content_to_write.len() - 2);
    content.insert(start_of_versions_index, format!("{}", new_entry.join("")));
    // Then, by last, write the content
    create_changelog(content, &new_version)
}
