use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;

use Search;

pub struct WordlistSearcher {
    search_term: String,
}

impl Search for WordlistSearcher {

    fn new(search_term: String) -> Self {
        Self {
            search_term: search_term,
        }
    }

    fn search(&self) -> Vec<String> {
        let path = env!("CARGO_MANIFEST_DIR").to_string() + "/src/words_alpha.txt";
        let word_list = lines_from_file(path);
        let mut candidates = Vec::new();
        for word in &word_list {
            if word.contains(self.search_term.as_str()) {
                candidates.push(word.to_string());
            }
        }
        candidates
    }

}

fn lines_from_file<P>(filename: P) -> Vec<String> where P: AsRef<Path> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

#[cfg(test)]
mod tests {

    use Search;
    use wordlist_searcher::WordlistSearcher;

    #[test]
    fn basic_search() {
        let searcher = WordlistSearcher::new("sss".to_string());
        assert_eq!(searcher.search(), ["asssembler", "bossship", "demigoddessship",
                                       "earlesss", "goddessship", "headmistressship",
                                       "passsaging", "patronessship"]);
    }

}

