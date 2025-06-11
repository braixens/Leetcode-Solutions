use std::collections::HashMap;

impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {

        //New map, Total Counter
        let mut stone_map = HashMap::new();
        let mut total = 0;

        //If character is in map, add 1 to current value
        //Else add it to map with 1 value
        for ch in stones.chars(){
            if stone_map.contains_key(&ch) {
                *stone_map.get_mut(&ch).unwrap() += 1;
            } else { 
                stone_map.insert(ch, 1);
            }
        }

        // For every character, check if its in the map
        //If it is add the value of that character
        // (How many times its seen)To total
        for jewel in jewels.chars() {
            if stone_map.contains_key(&jewel){
                total += *stone_map.get_mut(&jewel).unwrap();
            }

        }
        return total;
    }
}
