fn main() {
    let x = plus_one(5);
    println!("x = {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1 // ここは式です. なので, x + 1; とはしない.
}
