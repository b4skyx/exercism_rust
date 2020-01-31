use std::collections::HashSet;
use std::collections::HashMap;

fn frequency_hash( word: &str ) -> HashMap<char, u32>{
    let mut freq : HashMap<char, u32> = HashMap::new();
    for character in word.chars(){
        let counter = freq.entry(character).or_insert(0);
        *counter += 1;
    }
    freq
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut result = HashSet::new();
    let lowercased_word = word.to_lowercase();
    let word_hash = frequency_hash(&lowercased_word);
    for &possible in possible_anagrams.iter(){
        let lowercased_possible = possible.to_lowercase();
        if lowercased_possible == lowercased_word{
            continue;
        }
        else if word_hash == frequency_hash(&lowercased_possible){
                result.insert(possible);
        }
    }
    result
}
