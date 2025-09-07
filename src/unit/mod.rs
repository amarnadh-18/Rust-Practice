use std::mem::size_of_val;
pub fn run(){
    let unit: () = ();
    println!("{}", size_of_val(&unit)); 

    let random: (i32, u32) = (-5, 3);
    println!("{}", size_of_val(&random));
}