pub fn example(){
    // let x =5u32;
    // let y = {
    //     let x_squared = x*x;
    //     let x_cube = x_squared*x;

    //     x_cube + x_squared + x
    // };

    // let z = {
    //     2*x
    // };
    // println!("{}", x);
    // println!("{}",y);
    // println!("{}",z);

//     let v= {
//        let mut x = 1;
//        x += 2;
//        x
//    };

//    println!("{:?}", v);

    // let v = {
    //     let x = 3;
    //     x
    // };
    // println!("{}", v);

    let s = sum(1, 2);
    println!("{}", s);
}
fn sum(x: i32, y: i32) -> i32 {
    x + y
}