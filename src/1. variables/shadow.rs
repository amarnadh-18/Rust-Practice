fn main() {
    let mut x: i32 = 1;
    println!("{}",x);
    x = 7;
    println!("{}",x);
    // Shadowing and re-binding
    println!("{}", x);
    x += 3;


    let y = 4;
    println!("{}", y);
    // Shadowing
    let y = "I can also be bound to text!"; 
    println!("{} {}", x, y);
    println!("Success!");
}