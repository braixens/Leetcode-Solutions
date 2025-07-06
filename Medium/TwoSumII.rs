impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut l = 0;
    let mut r = numbers.len() - 1;
    
        if numbers[l] >= 0 {
         while numbers[l] > target {
                l += 1;
         }
        }
        while numbers[r] + numbers [l] != target {
         if numbers[l] + numbers[r] > target {
                r -= 1;
            } else {
                if numbers[l] + numbers[r] < target {
                 l += 1;
                }
            }
        } return vec![(l + 1 ) as i32, (r + 1) as i32]
    }
}
