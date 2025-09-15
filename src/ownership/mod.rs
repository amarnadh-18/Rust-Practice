pub fn run(){
    //let x = &String::from("Hello World");
    let x = String::from("Hello World");
    //let y = x.clone();
    //let y = x.as_str();
    let y = &x;
    println!("{}, {}", x, y);
}