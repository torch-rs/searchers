extern crate dirs;
extern crate search_candidate;
extern crate walkdir;

use self::search_candidate::SearchCandidate;
use self::walkdir::WalkDir;

use Search;

pub struct FilesSearcher;

fn search_directory(root_directory: String) -> Vec<String> {
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
            None => String::from("")
        };
        let mut result = Vec::new();
        for entry in &search_directory(root_path) {
            result.push(SearchCandidate::new(entry.clone(), entry.clone(), String::from("")));
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
        let candidates = search_directory(root_path);
        for i in 0..candidates.len() {
            println!("{}", candidates[i]);
            assert!(candidates[i].ends_with(actual_candidates[i]));
        }
    }

}
