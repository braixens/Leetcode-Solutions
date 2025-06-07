use std::collections::HashSet;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        // Create a Set to keep track of all numbers in vec 
        let mut set = HashSet::new();
        // Iterate through the nums list and adds every number to set
        for num in nums{
            if // Duplicate found
            set.contains(&num){
                return true
            } else {
                set.insert(num);
            }
        }
        false // Loop finishes, No duplicate
    }
} 
