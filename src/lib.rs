pub mod app_searcher;
pub mod files_searcher;
pub mod wordlist_searcher;

pub trait Search {

    fn search() -> Vec<String>;

}
