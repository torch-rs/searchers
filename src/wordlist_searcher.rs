use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;

use Search;

pub struct WordlistSearcher; 

impl Search for WordlistSearcher {

    fn search() -> Vec<String> {
        let path = env!("CARGO_MANIFEST_DIR").to_string() + "/src/words_alpha.txt";
        lines_from_file(path)
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
        assert_eq!(WordlistSearcher::search().len(), 370099);
    }

}
