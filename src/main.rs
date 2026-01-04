fn main() {
    let tup = (500,"hi",true);
    println!(" hey {}",tup.0);
    
    let (x,y,f) = tup;
    println!(" hey {}",f);

   
}
