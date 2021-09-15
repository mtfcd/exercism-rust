use std::collections::{HashSet, BTreeMap};

fn make_map(word: &str) -> BTreeMap<char, usize> {
    let mut chars = BTreeMap::new();
    for c in word.to_lowercase().chars() {
        *chars.entry(c).or_insert(0) += 1;
    }
    chars
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut res = HashSet::new();

    let letter = make_map(word);

    'b:
    for &w in possible_anagrams {
        if w.to_lowercase() == word.to_lowercase() {
            continue;
        }
        let candidate = make_map(w);
        if candidate.len() != letter.len() {
            continue;
        }
        for (k, v) in candidate {
            if letter.get(&k).is_none() || letter[&k] != v {
                continue 'b;
            }
        }
        res.insert(w);
        
    }

    return res
}
