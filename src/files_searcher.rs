extern crate dirs;
extern crate search_candidate;
extern crate walkdir;

use Search;
use self::search_candidate::SearchCandidate;
use self::walkdir::WalkDir;
use std::path::Path;

pub struct FilesSearcher;

fn search_directory(root_directory: &str) -> Vec<String> {
    let mut candidates = Vec::new();
    for entry in WalkDir::new(root_directory).contents_first(true).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_dir() {
            continue;
        }
        candidates.push(entry.path().to_string_lossy().into_owned());
    }
    candidates
}

impl Search for FilesSearcher {

    fn search() -> Vec<SearchCandidate> {
        let root_path = match dirs::home_dir() {
            Some(path) => path.to_string_lossy().into_owned(),
            None => String::new(),
        };
        let mut result = Vec::new();
        for entry in &search_directory(&root_path) {
            let file_ext = Path::new(&entry).extension();
            let mut icon_path = String::new();
            if let Some(file_ext) = file_ext {
                icon_path = format!("fiv-viv fiv-icon-{}", file_ext.to_string_lossy());
            }
            result.push(SearchCandidate::new(entry.clone(), entry.clone(), icon_path));
        }
        result
    }
    
}

#[cfg(test)]
mod tests {

    use files_searcher::search_directory;

    #[test]
    fn simple_search() {
        let root_path = format!("{}/src", env!("CARGO_MANIFEST_DIR"));
        let actual_candidates = ["windows_searcher.rs", "lib.rs", "files_searcher.rs", "words_alpha.txt",
                                 "app_searcher.rs", "wordlist_searcher.rs"];
        let candidates = search_directory(&root_path);
        for i in 0..candidates.len() {
            println!("{}", candidates[i]);
            assert!(candidates[i].ends_with(actual_candidates[i]));
        }
    }

}
