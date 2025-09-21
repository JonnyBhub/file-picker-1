// This file contains definitions for icons used to represent folders and files in the user interface.

pub const FILE_ICON: &str = "📄";   // Icon for files
pub const OPEN_FOLDER_ICON: &str = "📂"; // Icon for open folders
pub const CLOSED_FOLDER_ICON: &str = "📁"; // Icon for closed folders

pub fn get_icon(is_folder: bool, is_open: bool) -> &'static str {
    if is_folder {
        if is_open {
            OPEN_FOLDER_ICON
        } else {
            CLOSED_FOLDER_ICON
        }
    } else {
        FILE_ICON
    }
}
