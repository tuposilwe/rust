use std::sync::{Arc, mpsc,Mutex};
use std::thread;
use rayon::prelude::*;
use num::{ BigUint, One};
use std::time::Instant;


fn factorial(num: u32) -> BigUint{
    if num == 0 || num == 1{
        return BigUint::one();
    }else {
        (1..=num).map(BigUint::from).reduce(|acc ,x| acc * x).unwrap()
    }
}

fn multi_factor(num: u32) -> BigUint{
    if num == 0 || num == 1{
        return BigUint::one();
    }else {
        (1..=num).into_par_iter().map(BigUint::from).reduce(|| BigUint::one(),|acc,x| acc * x)
    }
}

fn main() {
    // let handle = thread::spawn(move || {
    //   println!("Hello from a thread!")
    // });

    // handle.join().unwrap();

    // println!("hello from main");

    // let v = [1, 2, 3];

    // let mut thread_handles = Vec::new();

    // for e in v {
    //     thread_handles.push(thread::spawn(move || println!("thread {}", e)));
    // }

    // println!("Main thread!");

    // for handle in thread_handles {
    //     handle.join().unwrap();
    // }

    // let (transmitter, receiver) = mpsc::channel();
    // let tx = transmitter.clone();

    // let val = String::from("Transmitting!");
    // thread::spawn(move || {
    //     transmitter.send(val).unwrap();
    // });

    // let msg = receiver.recv().unwrap();
    // println!("{}",msg);

    // std::thread::spawn(move || {
    //     let vec = vec![
    //         String::from("Transmitting"),
    //         String::from("From"),
    //         String::from("Original"),
    //     ];
    //     for val in vec {
    //         transmitter.send(val).unwrap();
    //     }
    // });

    // std::thread::spawn(move || {
    //     let vec = vec![
    //         String::from("Clone"),
    //         String::from("Is"),
    //         String::from("Transmitting"),
    //     ];
    //     for val in vec {
    //         tx.send(val).unwrap();
    //     }
    // });


    // for rec in receiver{
    //     println!("{}",rec);
    // }


    // let rc1 = Arc::new(String::from("Test"));
    // let rc2 = rc1.clone();

    // std::thread::spawn(move || {
    //     rc2;
    // });


    // let counter = Arc::new(Mutex::new(0));
    // let mut handles = vec![];

    // for _ in 0..8 {
    //     let counter = Arc::clone(&counter);
    //     let handle = std::thread::spawn(move || {
    //         let mut num = counter.lock().unwrap();
    //         *num += 1;
    //     });
         
    //     handles.push(handle);
    // }

    // for handle in handles {
    //     handle.join().unwrap();
    // }

    // println!("{}",counter.lock().unwrap());

    // println!("{}",factorial(10));


    let now = Instant::now();
    factorial(5000);
    println!("{:.2?}",now.elapsed());

    let now = Instant::now();
    multi_factor(5000);
    println!("{:.2?}",now.elapsed());

}
