fn main() {
    // print_phase("Print my argument");
    println!("{}",gcd(20, 5));
}

fn print_phase(phrase: &str) {
    println!("{}",phrase);
}

fn gcd(mut a: u64,mut b:u64) -> u64{
    while a != 0 {
        if a < b{
            let c = a;
            a = b;
            b = c;
        }
        a =  a % b;
    }
    b
}