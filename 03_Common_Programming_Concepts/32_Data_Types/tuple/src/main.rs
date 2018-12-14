fn main() {
    let tup: (i32, f64, u8) = (400, 34.324, 2);
    let (x, y, z) = tup;
    println!("x = {}", x);
    println!("x = {}", tup.0);
    println!("y = {}", y);
    println!("y = {}", tup.1);
    println!("z = {}", z);
    println!("z = {}", tup.2);
    println!("tup = {:?}", tup);
/*
    x = 400
    x = 400
    y = 34.324
    y = 34.324
    z = 2
    z = 2
    tup = (400, 34.324, 2)
    と表示.
*/
}
