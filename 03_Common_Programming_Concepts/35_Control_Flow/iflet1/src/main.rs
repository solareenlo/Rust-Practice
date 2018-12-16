fn main() {
    let condition = true;
    // letの後のif文では, 型を揃えないといけない.
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
}
