use std::rc::Rc;

fn main() {
    let t = (12, "eggs"); // created on the stack
    let b = Box::new(t); // created on the heap, but b was stored on the stack
    println!("{:?}", b);

    let x = 5;
    let y = &x;

    // assert_eq!(5,x);
    // assert_eq!(5,*y);

    println!("{:?}", *y);

    let s1 = Rc::new(String::from("Pointer"));
    let s2 = s1.clone();
    let s3 = s2.clone();
    println!("{}, {} , {}", s1.contains("Point"), s2, s3.contains("er"));
}
