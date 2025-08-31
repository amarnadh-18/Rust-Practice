fn main(){
    // let x = 5;
    // let y = 6;

    // assert_eq!(x, 5);
    // println!("Success {} {}",x ,y);

    // let mut x = 1;
    // x += 2;
    // let y = 1;
    // // println!("{}", x);

    // {
    //     let x = 10;
    //     let y = 5;
    //     println!("{} {}", x,y);
    // }
    // println!("{} {}",x, y);


// Only modify `assert_eq!` to make the `println!` work(print `42` in terminal)

    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x = 42;
    println!("{}", x); // Prints "42".


}