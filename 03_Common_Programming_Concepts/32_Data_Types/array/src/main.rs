fn main() {
    let a = [1, 2, 3];
    let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun",
                   "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    let first = b[0];
    let second = b[1];
    println!("a = {:?}", a);
    println!("months = {:?}", months);
    println!("b = {:?}", b);
    println!("b_first = {}", first);
    println!("b_second = {}", second);
/*
    a = [1, 2, 3]
    months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"]
    b = [1, 2, 3, 4, 5]
    b_first = 1
    b_second = 2
    と表示.
*/
}
