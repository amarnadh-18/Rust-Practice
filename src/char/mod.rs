use std::mem::size_of_val;
pub fn run() {
    let c1 = 'a';
    println!("{}",size_of_val(&c1)); 

    let c2 = '中';
    println!("{}",size_of_val(&c2)); 

} 