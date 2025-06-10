use std::collections::HashSet;
impl Solution {
    pub fn longest_consecutive(mut nums: Vec<i32>) -> i32 {

        //Keep track of current sequence and highest sequence
        let mut highest = 0;
        let mut current = 0;
        let mut set: HashSet<i32> = HashSet::new();

      // Sort list so when put into map its in order
        nums.sort();

      //This line only for 1 test case. need to fix code cause it skips one
        if nums.is_empty(){
            return 0;
        } 

          //If prev is in the set, add one to counter and check for highest
        else {
        for num in &nums {
            if !set.contains(num){
                if set.contains(&(num - 1)) {
                 set.insert(*num);
                 current += 1;
                 if current > highest {
                       highest = current;
                    }
                } //
                // if not in set insert and start new counter
                else {
                 set.insert(*num);
                 current = 0;
                }
            }
        }
          //?? code was glitching but this fixed it??
        highest + 1
    }
}
}
