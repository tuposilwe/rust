use core::option::Option::{self, None, Some};

enum Pet {dog,cat,fish}

impl Pet {
    fn what_am_i(self) ->&'static str {
        match self {
            Pet::dog => "I am a dog",
            Pet::cat => "I am a cat",
            Pet::fish => "I am a fish",
            
        }
    }
}

enum IpAddrKind {
    v4(String),
    v6
}

struct  IpAddr{
    kind: IpAddrKind,
    address: String
}

fn main() {
    let my_pet = Pet::dog;
    println!("{}",my_pet.what_am_i());

    let home = IpAddrKind::v4(String::from("127.0.0.1"));
    
    let loopBack = IpAddr{
        kind: IpAddrKind::v6,
        address:String::from("::1")
    };

    let some_number = Some(5);
    let some_string = Some("a string");
    let nothing : Option<i8> = None;

    let x: i32 = 5;
    let y: Option<i32> = Some(5);
    // let sum = x + y;

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?}",none);

    what_pet("Dog");
    what_pet("Cat");
    what_pet("Cow");
}

fn plus_one(x: Option<i32>) -> Option<i32>{
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}

fn what_pet(input: &str){
    match input {
        "Dog" => println!("I have a dog!"),
        "Fish" => println!("I have a fish!"),
        "Cat" => println!("I have a cat!"),
        _ => println!("I have no clue what pet you have")
        
    }
}