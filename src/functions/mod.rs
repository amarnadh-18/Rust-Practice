//use core::{panic, time};

pub fn run(){
    // let (x, y) = (1, 2);
    // let s = sum(x, y);
    // println!("{}", s);
    // print();
    //never_return();
    //println!("Fail");
    let b = false;

    let _v = match b{
        true => 1,
        false => {
            println!("Sucess");
            panic!("We have no value for 'false', but we can panic");
        }
    };
    println!("Exercise Failed if printing out this line!");
}
// fn sum(x: i32, y:i32) -> i32{
//     x + y 
// }
// fn print() -> (){
//     println!("Hello World");
// }
// fn never_return() -> !{
//     panic!("I return nothing")
// }
//use std:: thread;
//use std::time;
// fn never_return() -> !{
//     loop{
//         println!("I return nothing");
//         thread::sleep(time::Duration::from_secs(1))
//     }
// }
// fn get_option(tp: u8) -> Option<i32>{
//     match tp {
//         1 => {
//             //TODO

//         }
//         _ => {
//             //TODO

//         }
//     };

//     never_return_fn()
// }
// fn never_return_fn() -> !{
//     //panic!("This function doesnt return anything")
//     //unimplemented!()
//     //todo!()
//     loop {
//         std::thread::sleep(std::time::Duration::from_secs(1))
//     }
// }