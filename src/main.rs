fn main() {
    let name  = String::from("Tyler");
    let course = "Rust".to_string();
    let new_name = name.replace("Ty","UI");

    println!("{}",name);
    println!("{}",course);
    println!("{}",new_name);
    
    // &str = "string slice" or "stir"
    let str1  = "hello"; //&str
    let str2 = str1.to_string();
    let str3 = &str2;
    
    println!("{}",str1);
    println!("{}",str2);
    println!("{}",str3);
    
    // compare string == != (does not equal)
    println!("{}","ONE".to_lowercase() == "one");
}
