use std::thread;

fn main() {
    // let handle = thread::spawn(move || {
    //   println!("Hello from a thread!")
    // });

    // handle.join().unwrap();

    // println!("hello from main");

    let v = [1, 2, 3];

    let mut thread_handles = Vec::new();

    for e in v {
        thread_handles.push(thread::spawn(move || println!("thread {}", e)));
    }

    println!("Main thread!");

    for handle in thread_handles {
        handle.join().unwrap();
    }
}
