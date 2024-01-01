use std::collections::HashMap;

pub fn group_anagram(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut result = Vec::new();

    for word in strs {
        let mut word_hash = HashMap::new();

        for character in word.chars() {
            if word_hash.contains_key(&character){
                let mut count = word_hash.get_mut(&character).unwrap();
                *count +=1;
            }else{
                word_hash.insert(&character, 1);
            }
        }


    }

    unimplemented!
}
