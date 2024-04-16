use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    // Read words from file
    let words = read_words("data/words.txt");
    let mut anagrams: HashMap<String, Vec<String>> = HashMap::new();
    for word in words {
        let mut letters: Vec<char> = word.chars().collect();
        letters.sort();
        let sorted_letters: String = letters.iter().collect();

        anagrams
            .entry(sorted_letters)
            .or_insert(Vec::new())
            .push(word.to_string());
    }

    anagrams.retain(|_, words| words.len() > 2);

    println!("{} entries", anagrams.len());
    for _ in 1..100 {
        let anagrams_copy = anagrams.clone();
        let max_anagrams = anagrams_copy.values().max_by_key(|v| v[0].len()).unwrap();
        println!("{}: {:?}", max_anagrams[0].len(), max_anagrams);
        anagrams.retain(|_, words| words != max_anagrams);
    }
}

fn read_words(filename: &str) -> Vec<String> {
    let mut words = Vec::new();
    let file = File::open(filename).expect("Could not open file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.expect("Could not read line");
        words.push(line);
    }
    words
}
