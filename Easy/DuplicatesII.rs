use std::collections::HashMap;

impl Solution {

    //Function checks for duplicates within k distance
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        //Makes a new map
        let mut map = HashMap::new();

        // Iterates over the nums list, keeps track of index + number
        for (i,num) in nums.iter().enumerate(){
            
            //If number already in map, compare distances to the one inputted
            if let Some(&last_index) = map.get(num) {
                if  (i - last_index) as usize <= k as usize {
                    return true
                }
            }
            // If distances are ok, replace the number with new index
            map.insert(*num, i);
        }
        //If loop finishes, no duplicates 
        false
    }
}
