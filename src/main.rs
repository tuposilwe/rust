fn main() {
    let _var = 1;
    let mut s = "hello".to_string();
    s.push_str(" ,world");
    // println!("{}", s);

    let x = vec!["tyler".to_string()];
    let _y = x.clone();
    // println!("{:?}",x);

    let a = 2;
    let _b = a;

    // println!("a = {}, b = {}",a,b);

    let _s = String::from("Takes");
    
    takes_onwership(_s); // give ownership to the function

    let val =3;
    make_copy(val);

    let str1:String = give_onwership();
    println!("{}", str1);

    let str3:String = take_and_give(str1);
    println!("{}", str3);
    
    let mut my_name = String::from("Deskillz47");
    change_string(&mut my_name);
    println!("{}", my_name);

}

fn takes_onwership(s: String) {
    let strin = s;
    println!("{}", strin);
}

fn make_copy(s: i32) {
    let strin = s;
    println!("{}", strin);
}

fn give_onwership() -> String {
    "given".to_string()
}

fn take_and_give(str2: String) -> String {
    str2
}

fn change_string(some_string:&mut String){
    some_string.push_str(",rudiger");
}