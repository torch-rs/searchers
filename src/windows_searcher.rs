extern crate raise_window;

use Search;

pub struct WindowsSearcher;

impl Search for WindowsSearcher {

    fn search() -> Vec<String> {
        if let Ok(result) = raise_window::get_all_windows_by_name() {
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
