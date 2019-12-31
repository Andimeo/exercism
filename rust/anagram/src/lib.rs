use std::collections::{HashMap, HashSet};

fn to_map(word: &str) -> HashMap<String, i32> {
    let mut map = HashMap::new();
    word.chars().for_each(|c: char| {
        let d = c.to_lowercase().collect::<String>();
        match map.get_mut(&d) {
            None => {
                map.insert(d, 1);
            }
            Some(count) => {
                *count += 1;
            }
        }
    });
    map
}

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let map1 = to_map(word);
    possible_anagrams
        .iter()
        .filter(|candidate| {
            candidate.len() == word.len() && candidate.to_lowercase() != word.to_lowercase()
        })
        .filter(|candidate| {
            let mut map2 = map1.clone();
            for c in candidate.chars() {
                let d = c.to_lowercase().collect::<String>();
                match map2.get_mut(&d) {
                    None | Some(0) => {
                        return false;
                    }
                    Some(count) => *count -= 1,
                }
            }
            true
        })
        .map(|candidate| *candidate)
        .collect()
}
