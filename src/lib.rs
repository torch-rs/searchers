pub mod wordlist_searcher;

pub trait Search {

    fn new(search_term: String) -> Self;
    fn search(&self) -> Vec<String>;

}
