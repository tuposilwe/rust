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
}