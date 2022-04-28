
pub fn bubble_swap(mut nums: Vec<i32>) -> Vec<i32>{

    //bubble swap works by swapping the number if the number before it is smaller
    for i in 0..nums.len(){
        for j in 1..nums.len(){
            let curr = nums[j];
            let prev = nums[j-1];

            //If current is less than previous swap
            if(curr<prev){
                //put current in previous
                nums[j-1] = curr;
                nums[j] = prev;
            }
        }
    }

    nums
}