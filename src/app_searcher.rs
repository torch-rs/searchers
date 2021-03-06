extern crate dirs;
extern crate inflector;
extern crate fs2;
extern crate search_candidate;
extern crate walkdir;

use self::fs2::FileExt;
use self::inflector::Inflector;
use self::search_candidate::SearchCandidate;
use self::walkdir::WalkDir;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

use Search;

pub struct AppSearcher;

fn get_line_from_file(filename: &str, line_prefix: &str) -> Option<String> {
    match File::open(filename) {
        Ok(file) => {
            file.lock_exclusive().unwrap();
            let buf = BufReader::new(&file);
            let content: Vec<String> = buf.lines()
                .map(|l| l.expect("Could not parse line"))
                .collect();
            file.unlock().unwrap();
            for line in &content {
                if line.starts_with(line_prefix) {
                    return Some(line.clone());
                }
            }
            None
        }
        Err(_err) => None
    }
}

fn get_icon_dir(icon_root_dir: &str) -> Option<String> {
    let size_dir = vec!["512x512", "64x64", "48x48", "32x32", "24x24", "16x16"];
    for size in &size_dir {
        let test_icon_dir = format!("{}/{}/apps", icon_root_dir, size);
        if Path::new(&test_icon_dir).exists() {
            return Some(test_icon_dir.clone());
        }
    }
    None
}

fn try_get_icon_path(root_icon_dir: &str, icon: &str) -> Result<String, ()> {
    if let Some(icon_dir) = get_icon_dir(root_icon_dir) {
        let test_png_icon_path = format!("{}/{}.png", icon_dir, icon);
        let test_svg_icon_path = format!("{}/{}.svg", icon_dir, icon);
        if icon.contains("/") {
            return Ok(icon.to_string());
        } else if Path::new(&test_png_icon_path).exists() {
            return Ok(test_png_icon_path.clone());
        } else if Path::new(&test_svg_icon_path).exists() {
            return Ok(test_svg_icon_path.clone());
        } else {
            return Err(());
        }
    }
    return Err(());
}

fn get_icon_path_from_desktop_file(desktop_filename: &str) -> String {
    let homedir = match dirs::home_dir() {
        Some(path) => path.to_string_lossy().into_owned(),
        None => String::from("")
    };
    let mut icon = String::new();
    if let Some(line) = get_line_from_file(desktop_filename, "Icon=") {
        icon = line[line.find("=").unwrap()+1..].to_string();
        let png_pixmaps_path = format!("/usr/share/pixmaps/{}.png", icon);
        let svg_pixmaps_path = format!("/usr/share/pixmaps/{}.svg", icon);
        if Path::new(&png_pixmaps_path).exists() {
            return png_pixmaps_path.clone();
        }
        if Path::new(&svg_pixmaps_path).exists() {
            return svg_pixmaps_path.clone();
        }
    }
    if let Some(line) = get_line_from_file(&format!("{}/.gtkrc-2.0", homedir), "gtk-icon-theme-name=\"") {
        let root_icon_path = format!("/usr/share/icons/{}", line[line.find("\"").unwrap()+1..line.len()-1].to_string());
        match try_get_icon_path(&root_icon_path, &icon) {
            Ok(icon_path) => return icon_path,
            Err(_err) => {},
        }
        match try_get_icon_path("/usr/share/icons/hicolor", &icon) {
            Ok(icon_path) => return icon_path,
            Err(_err) => return String::new(),
        }
    }
    String::new()
}

fn get_directory_content(root_path: &str) -> Vec<String> {
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
    let user_app_directory = &format!("{}/.local/share/applications", homedir);
    let main_root_app_directory = "/usr/share/applications";
    let secondary_root_app_directory = "/usr/local/share/applications";

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
            let icon_path = get_icon_path_from_desktop_file(&candidate);
            candidates_vec.push(SearchCandidate::new(filename.to_string_lossy().into_owned(),
                                                     filename.to_string_lossy().into_owned().to_title_case(),
                                                     icon_path));
        }
    }
    candidates_vec
}

fn search_macos() -> Vec<SearchCandidate> {
    let homedir = match dirs::home_dir() {
        Some(path) => path.to_string_lossy().into_owned(),
        None => String::from("")
    };
    let user_app_directory = &format!("{}/Applications", homedir);
    let main_root_app_directory = "/Applications";

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

    use app_searcher::AppSearcher;
    use Search;

    #[test]
    fn verify_found_all_apps() {
        let candidates = AppSearcher::search();
        assert_eq!(candidates.len(), 94);
    }

}
