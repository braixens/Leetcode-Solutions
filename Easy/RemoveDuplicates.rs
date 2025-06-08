use std::collections::HashSet;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> usize {
        // initializes a hashmap
        let mut set = HashSet::new();
        let mut k = 0;
        // Gives number of times to iterate
        let len = nums.len();
        // Loops through list and adds all to set 
        for i in 0..len {
            let num = nums[i];
            if set.insert(num) {
            // replaces on it index as goes on +adds counter
            // of  unique numbers
            nums[k] = num;    
            k += 1;
            }
        }
        //make the list only contain the amount of 
        //unique elements
        nums.truncate(k);
        return k
    }
}
