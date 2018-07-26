extern crate search_candidate;

pub mod app_searcher;
pub mod files_searcher;
pub mod windows_searcher;
pub mod wordlist_searcher;

use self::search_candidate::SearchCandidate;

pub trait Search {

    fn search() -> Vec<SearchCandidate>;

}
