fn main() {
    let s = String::from("hello");
    takes_owership(s); // ここでsはスコープを抜けてる.
    // println!("{}", s); // これはエラーになる. sの所有権が関数に渡ったから.

    let x = 5;
    makes_copy(x);
    println!("{}", x); // これはエラーにならない. xはスタック領域にいるから.
} // ここでxはスコープを抜ける. sはもうすでにスコープを抜けてる.

fn takes_owership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
