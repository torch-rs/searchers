extern crate walkdir;
extern crate dirs;

use self::walkdir::WalkDir;

use Search;

pub struct FilesSearcher;

impl Search for FilesSearcher {

    fn search() -> Vec<String> {
        // the following line is used during testing
        // let root_path = format!("{}/src", env!("CARGO_MANIFEST_DIR"));
        let root_path = match dirs::home_dir() {
            Some(path) => path.to_string_lossy().into_owned(),
            None => String::from("")
        };
        let mut candidates = Vec::new();
        for entry in WalkDir::new(root_path).contents_first(true).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_dir() {
                continue;
            }
            candidates.push(entry.path().to_string_lossy().into_owned());
        }
        candidates
    }
    
}

#[cfg(test)]
mod tests {

    use Search;
    use files_searcher::FilesSearcher;

    #[test]
    fn simple_search() {
        let actual_candidates = ["lib.rs", "files_searcher.rs", "words_alpha.txt", "wordlist_searcher.rs"];
        let candidates = FilesSearcher::search();
        for i in 0..candidates.len() {
            println!("{}", candidates[i]);
            assert!(candidates[i].ends_with(actual_candidates[i]));
        }
    }

}
