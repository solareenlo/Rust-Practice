fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);
    // The value of x is: 12 と表示.

    // shadowingは型も変更できる.
    // mutとはちょっと違う.
    // mutは型の変更はできない.
    let spaces = "    ";
    let spaces = spaces.len();
    println!("{}", spaces);
    // 4 と表示.
}
