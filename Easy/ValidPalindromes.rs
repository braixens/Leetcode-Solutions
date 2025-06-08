impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        
        // Makes a vector with ONLY lowercase alphanumeric characters
        let mut letters: Vec<char> = 
        s.to_lowercase().chars().filter(|c| c.is_alphanumeric()).collect();
        
        // 2 pointers to compare opposite sides
        let mut left = 0;
        let mut right = letters.len().saturating_sub(1);

        //if opposite sides are = it continues loop otherwise its not a palindrome
        while left < right {
            if letters[left] != letters[right] {
                return false;
            } else {
                left += 1;
                right -= 1;
            }
        }
    // Returns true if finishes loop
     true
    }
}
