extern crate raise_window;
extern crate search_candidate;

use Search;
use self::search_candidate::SearchCandidate;

pub struct WindowsSearcher;

impl Search for WindowsSearcher {

    fn search() -> Vec<SearchCandidate> {
        if let Ok(window_vec) = raise_window::get_all_windows_by_name() {
            let mut result = Vec::new();
            for window in &window_vec {
                result.push(SearchCandidate::new(window.clone(), window.clone(), String::from("")));
            }
            return result;
        }
        Vec::new()
    }

}

#[cfg(test)]
mod tests {

    use Search;
    use windows_searcher::WindowsSearcher;

    #[test]
    fn verify_found_all_windows() {
        let candidates = WindowsSearcher::search();
        assert_eq!(candidates.len(), 3);
    }

}
