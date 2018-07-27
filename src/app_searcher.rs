extern crate dirs;
extern crate inflector;
extern crate search_candidate;
extern crate walkdir;

use self::inflector::Inflector;
use self::search_candidate::SearchCandidate;
use self::walkdir::WalkDir;
use std::collections::HashSet;
use std::path::Path;

use Search;

pub struct AppSearcher;

fn get_directory_content(root_path: String) -> Vec<String> {
    let mut dir_contents = Vec::new();
    for entry in WalkDir::new(root_path).contents_first(true).into_iter().filter_map(|e| e.ok()) {
        if (cfg!(target_os="macos") && !entry.path().to_string_lossy().into_owned().ends_with(".app")) ||
            (!cfg!(target_os="macos") && entry.file_type().is_dir()) {
                continue;
            }
        dir_contents.push(entry.path().to_string_lossy().into_owned());
    }
    dir_contents
}

fn search_linux() -> Vec<SearchCandidate> {
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
        let path = Path::new(&candidate);
        if let Some(filename) = path.file_stem() {
            candidates_vec.push(SearchCandidate::new(filename.clone().to_string_lossy().into_owned(),
                                                     filename.clone().to_string_lossy().into_owned().to_title_case(),
                                                     String::new()));
        }
    }
    candidates_vec
}

fn search_macos() -> Vec<SearchCandidate> {
    let homedir = match dirs::home_dir() {
        Some(path) => path.to_string_lossy().into_owned(),
        None => String::from("")
    };
    let user_app_directory = String::from(format!("{}/Applications", homedir));
    let main_root_app_directory = String::from("/Applications");

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

    let mut candidates_vec = Vec::new();
    for candidate in candidates {
        candidates_vec.push(SearchCandidate::new(candidate.clone(), candidate.clone().to_title_case(), String::new()));
    }
    candidates_vec
}

fn search_windows() -> Vec<SearchCandidate> {
    vec![]
}

impl Search for AppSearcher {

    fn search() -> Vec<SearchCandidate> {
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
        assert_eq!(candidates.len(), 99);
    }

}
