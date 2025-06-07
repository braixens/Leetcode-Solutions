use std::collections::HashMap;

impl Solution {
// Check if 2 words are anagrams > Return true/false
    pub fn is_anagram(s: String, t: String) -> bool {
        // Initializes 2 separate hashmaps for each word
     let mut s_map = HashMap::new();
     let mut t_map = HashMap::new();
    
// These both add the words to the map + adds each duplicate letter
        for ch in s.chars(){
         if s_map.contains_key(&ch) {
             *s_map.get_mut(&ch).unwrap() += 1;
         } else { 
             s_map.insert(ch, 1);
     }
    }   
        for ch in t.chars(){
             if t_map.contains_key(&ch) {
                *t_map.get_mut(&ch).unwrap() += 1;
                 } else { 
                  t_map.insert(ch, 1);
         }
        
     }
        // Will return true or false
        s_map == t_map
    }
}
