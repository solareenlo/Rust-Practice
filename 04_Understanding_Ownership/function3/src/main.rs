fn main() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    // println!("s1: {}", s1); // s1の所有権が関数に移ったのでエラーになる.
    // s1の文字列の長さを知りたいだけなのに, 所有権移譲があるので,
    // s1の所有権をまたもとに戻さないといけなくなる.
    // これは面倒なので参照と借用を用いる.
    // 次のコード(42のディレクトリ)に例があります.
    println!("The length of '{}' is '{}'.", s2, len); // The length of 'hello' is '5'. と表示.
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
