fn main(){
    //solution add mut in the parenthesis
    //let (mut x, y) = (1,2);
    // let (mut x, y) = (1, 2); //This is tuple
    // println!("{} {}", x, y);
    // x += 2;

    // assert_eq!(x, 3);
    // assert_eq!(y, 2);

    // println!("{} {}", x, y);

    let (x, y);
    (x,..) = (3, 4);
    [.., y] = [1, 2];
    println!("{:?}", (x, y));
    // Fill the blank to make the code work
    assert_eq!([x,y], [3,2]);

    println!("Success!");
}