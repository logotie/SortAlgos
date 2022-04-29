use std::time::Instant;

pub fn bubble_swap(nums: &Vec<i32>) -> Vec<i32>{

    let now = Instant::now();

    let mut temp = nums.to_owned();

    //bubble swap works by swapping the number if the number before it is smaller
    for i in 0..temp.len(){
        for j in 1..temp.len(){
            let curr = temp[j];
            let prev = temp[j-1];

            //If current is less than previous swap
            if(curr<prev){
                //put current in previous
                temp[j-1] = curr;
                temp[j] = prev;
            }
        }
    }

    let elapsed = now.elapsed();
    println!("Time taken to run bubble swap is: {:.2?}", elapsed);

    temp
}

pub fn insertion_sort(nums: &Vec<i32>) -> Vec<i32>{

    let now = Instant::now();

    let mut temp = nums.to_owned();

    for i in 0..temp.len(){
        let mut idx = i;
        while(idx > 0 && temp[idx-1] >= temp[idx]){
            let t = temp[idx-1];
            temp[idx-1] = temp[idx];
            temp[idx] = t;
            idx = idx-1;
        }
    }

    let elapsed = now.elapsed();
    println!("Time taken to run insertion sort is: {:.2?}", elapsed);

    temp
}