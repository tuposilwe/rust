fn main() {
    let v:Vec<i32> = (0..5).collect();
    println!("{:?}",v);
    
    let sv: &[i32] = &v[2..4];
    println!("{:?}",sv);
}
