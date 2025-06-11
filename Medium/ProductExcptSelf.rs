// Ok this code looks a bit WONKY so BEAR WITH MEEE

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {

    // These variables basically
    //Creates answer vector
    //Creates Product of all numbers
    //Makes a variable incase a 0 in list
    let mut answer: Vec<i32> = Vec::new();
    let product: i32 = nums.iter().product();
    let mut product_wo_0 = 0;

    // If there is NO zero in list this will run
    // Just pushes the product divided by the number
    //Which gives us the product (EXCEPT THE NUMBER DIVIDED !!)
    if product != 0 {
        for num in nums.iter() {
            answer.push(product / *num);
        }

    // This will run if theres 2 or MORE ZEROS
    //Which means all numbers would be 0
    //Replaces each element with a 0
    } else {
        if nums.iter().filter(|&&number| number == 0).count() >= 2 { 
            for _ in nums.iter() {
                answer.push(0);
            }

        // Now this will run if theres just ONE 0
        // this first piece unused, scrapped that idea
        //forgot to comment it out but whatever, leetcode 
        //compiler didnt tell me it was unused

        //This piece give us the index that the 0 is in lol
        } else {
            let position_0: usize = nums.iter().position(|&number| number == 0).unwrap();

          
            // !! the FILTER filters out the numbers, 
            //will only filter if it satisfied the 
            //NOT 0 equality !! will only use numbers that satisfy
            product_wo_0 = nums.iter().filter(|&&number| number != 0).product();
          
          // For every number in the list,
            //Checks if number isnt 0
            //Will put in a 0 cause theres a 0 somewhere else
            for num in nums.iter() {
                if *num != 0{
                answer.push(0);}
                else {
                    //Since only 1 zero itll only be 
                    // pushed once
                    answer.push(product_wo_0);
                }

            }
        }
    }
    return answer;
}
}
