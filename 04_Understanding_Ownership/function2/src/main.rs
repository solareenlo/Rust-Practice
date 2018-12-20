fn main() {
    let s1 = gives_ownership(); // s1は関数gives_ownership()から所有権を移譲される.
    println!("s1: {}", s1); // s1: hello と表示.
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2); // 関数を通してs2の所有権がs3にmoveした.
    // println!("s2: {}", s2); // これはエラーになる.
    println!("s3: {}", s3); // s3: hello と表示.
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string // some_stringは戻り値として所有権を移譲する.
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
