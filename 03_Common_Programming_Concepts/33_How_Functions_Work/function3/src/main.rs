fn main() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    let z = {3 + 1};

    println!("x = {}", x);
    println!("y = {}", y);
    println!("z = {}", z)
}
