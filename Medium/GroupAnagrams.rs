use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>>{

        //Makes new hashmap
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        //For every word in the input,  split into letters
        //Then sort the letters and make it the map key
        //So every matching key would be an anagram
        for word in &strs {
            let mut letters: Vec<char> = word.chars().collect();
            letters.sort();
            let sorted: String = letters.into_iter().collect();
            // If keys match, add word to the vector
            //Stored in the value 
            if let Some(value) = map.get_mut(&sorted) {
                value.push(word.clone());
            } else {
                //If no key, make new key-value pair 
                map.insert(sorted, vec![word.clone()]);
              }   
        }
        //Grabs JUST the values and adds each to a vector 
        //of strings and returns it
        let lists: Vec<Vec<String>> = map.values().cloned().collect();
        return lists
        
    }
}
