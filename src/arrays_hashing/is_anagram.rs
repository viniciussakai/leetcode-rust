use std::collections::HashMap;

pub fn is_anagram(s: String, t: String) -> bool {
    let mut first_word_hash: HashMap<char, i32> = HashMap::new();
    let mut second_word_hash: HashMap<char, i32> = HashMap::new();

    for char in s.chars() {
        if first_word_hash.contains_key(&char) {
            *first_word_hash.get_mut(&char).unwrap() += 1;
        } else {
            first_word_hash.insert(char, 1);
        }
    }

    for char in t.chars() {
        if second_word_hash.contains_key(&char) {
            *second_word_hash.get_mut(&char).unwrap() += 1;
        } else {
            second_word_hash.insert(char, 1);
        }
    }

    if second_word_hash ==  first_word_hash {
        return true;
    }

    return false;
}

