use rand::prelude::*;


fn main (){   
 let mut nums: Vec<i32> = vec![];

    nums.push(1);
    nums.push(2);
    nums.push(3);

    let pop = nums.pop(); //Option<T>, return None or Some(T)

    println!("{:?}", pop);

    let two = nums[1]; //copy
    // &nums[1], creates a reference if copy is not available

    println!("{}", two);

    let one = nums.first(); // return an Option<T>, so None if vec is empty, or Some<T> is [0]
    println!("{:?}", one);

    // .last
    // .first_mut and .last_mut, so will borrow mutuable references

    println!("{}", nums.len()); // return a value of the length
    println!("{}", nums.is_empty()); //bool 

    nums.insert(0, 10);
    nums.insert(3, 12);
    nums.insert(2, 25);

    nums.remove(3);

    nums.sort();
    println!("{:?}", nums);

    nums.reverse();
    println!("{:?}", nums);

    nums.shuffle(&mut rand::rng());
    println!("{:?}", nums);

}