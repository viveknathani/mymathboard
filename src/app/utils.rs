use std::path::Path;

pub fn get_board_name(file_path: Option<String>, board_has_unsaved_changes: bool) -> String {
    let mut result = "untitled".to_string();

    if let Some(path) = file_path {
        if let Some(file_name) = Path::new(&path).file_stem() {
            result = file_name.to_string_lossy().into_owned();
        }
    }

    if board_has_unsaved_changes {
        result += "*";
    }

    result
}
