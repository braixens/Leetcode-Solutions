impl Solution {
    pub fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {
        // Initializes a new vector for answer
        let mut indexes = Vec::new();
        
        //For every word, if the word has the letter given as x
        //Add the index to the new list
        //i is usize so cast as i32 when pushed to match type
        for i in 0..words.len(){
            let word = &words[i];
            if word.contains(x){
                indexes.push(i as i32)
            }
        }
        return indexes;
    }
}
