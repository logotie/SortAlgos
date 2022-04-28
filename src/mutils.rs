use rand::prelude::*;

//Generate the amount of random numbers
pub fn gen_random_numbers(amount: i32) -> Vec<i32>{

    //Create instance of random number generator
    let mut rng = rand::thread_rng();

    //declare a vector to store 32 bit integers
    let mut vec:Vec<i32> = Vec::new();

    for n in 1..amount {
        let rNo =  rng.gen::<i32>();
        vec.push(rNo);
    }

    vec
}