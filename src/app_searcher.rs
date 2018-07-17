extern crate walkdir;
extern crate dirs;

use self::walkdir::WalkDir;
use std::collections::HashSet;
use std::path::Path;

use Search;

pub struct AppSearcher;

fn get_directory_content(root_path: String) -> Vec<String> {
    let mut dir_contents = Vec::new();
    for entry in WalkDir::new(root_path).contents_first(true).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_dir() {
            continue;
        }
        dir_contents.push(entry.path().to_string_lossy().into_owned());
    }
    dir_contents
}

fn search_linux() -> Vec<String> {
    let homedir = match dirs::home_dir() {
        Some(path) => path.to_string_lossy().into_owned(),
        None => String::from("")
    };
    let user_app_directory = String::from(format!("{}/.local/share/applications", homedir));
    let main_root_app_directory = String::from("/usr/share/applications");
    let secondary_root_app_directory = String::from("/usr/local/share/applications");

    let mut candidates: HashSet<String> = HashSet::new();

    if Path::new(&user_app_directory).exists() {
        for candidate in get_directory_content(user_app_directory) {
            candidates.insert(candidate);
        }
    }
    if Path::new(&main_root_app_directory).exists() {
        for candidate in get_directory_content(main_root_app_directory) {
            candidates.insert(candidate);
        }
    }
    if Path::new(&secondary_root_app_directory).exists() {
        for candidate in get_directory_content(secondary_root_app_directory) {
            candidates.insert(candidate);
        }
    }

    let mut candidates_vec = Vec::new();
    for candidate in candidates {
        candidates_vec.push(candidate);
    }
    candidates_vec
}

fn search_macos() -> Vec<String> {
    vec![]
}

fn search_windows() -> Vec<String> {
    vec![]
}

impl Search for AppSearcher {

    fn search() -> Vec<String> {
        if cfg!(target_os="linux") {
            return search_linux();
        } else if cfg!(target_os="macos") {
            return search_macos();
        } else if cfg!(target_os="windows") {
            return search_windows();
        } else {
            return Vec::new();
        }
    }

}

#[cfg(test)]
mod tests {

    use Search;
    use app_searcher::AppSearcher;

    #[test]
    fn verify_found_all_apps() {
        let candidates = AppSearcher::search();
        assert_eq!(candidates.len(), 94);
    }

}
