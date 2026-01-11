struct User{
    active: bool,
    username: String,
    sign_in_count: u32
}

struct Coordinates(i32,i32,i32);
struct UnitStruct;
struct Square{
    width: u32,
    height: u32
}

impl Square {
    fn area(&self) -> u32{
        self.width * self.height
    }

    fn whats_my_width(&self) -> u32 {
        self.width
     }

    fn change_width(&mut self,new_width: u32){
        self.width = new_width;
    }
}


struct MyString<'a>{
    text: &'a str
}

fn main() {
    let user1 = User{
        active: true,
        username: String::from("Deskillz47"),
        sign_in_count: 0
    };

    println!("{}",user1.username);
    
    let user2 = build_user(String::from("Rudiger"));
    println!("{}",user2.username);

    let cords = Coordinates(1,2,3);

    let mut sq = Square{width:5,height:7};
    
    println!("{}",sq.area());
    println!("{}",sq.whats_my_width());
    sq.change_width(40);
    println!("{}",sq.whats_my_width());
    println!("{}",sq.area());
    
    let r;
    
    {
        let x = 5;
        r = x;
    }

    println!("lifetime {}",r);
    
    let str1 = String::from("This is my string");
    let x  = MyString{text: str1.as_str()};
    // println!("{:?}",x);



}

fn build_user(username: String) -> User{
    User {
         active: true,
         username,
         sign_in_count: 1
    }
}

fn example<'a>(x: &'a str) -> &'a str{ // 'a = lifetime
    x
}