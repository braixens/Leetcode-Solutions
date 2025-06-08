use std::collections::HashMap;
impl Solution {
        pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
            //initializes a new hashmap
            let mut map = HashMap::new();
            //For each number in list, get index and corresponding number
            for (i, num) in nums.iter().enumerate() {
                // If key found, return the current index, and the complements index
                let complement = target - *num;
                if map.contains_key(&complement) {
                let j = map[&complement]; 
                return vec![i as i32, j as i32] 
           } map.insert(*num, i);
        } //If no solution found
            return vec![]
    }
}
