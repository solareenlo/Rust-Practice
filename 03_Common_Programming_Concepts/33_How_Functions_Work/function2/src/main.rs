fn main() {
    another_function(5);
    another_function2(1, 2);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn another_function2(x: i32, y: i32) {
    println!("x is: {}", x);
    println!("y is: {}", y);
}
