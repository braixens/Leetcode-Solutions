// Pretty slow solution O(n^2) 
// Will rewrite using hashing or something


impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        
        //Make variable for total pairs
        let mut total_pairs = 0;

        //For every number, check the following numbers and if
        //It finds a match itll add 1 to the pair counter

      //Both for loops do the same thing just typed
      //Different ways vvvvv
        for (i, _) in nums.iter().enumerate(){
            for j in i..nums.len() {
                if i < j && nums[i] == nums[j]{
                total_pairs += 1
                }
            }
        }
        return total_pairs
    }
}
