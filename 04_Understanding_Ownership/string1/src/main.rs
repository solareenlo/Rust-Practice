fn main() {
    let s = String::from("hello");
    println!("{}", s); // hello と表示

    let mut st = String::from("hello");
    st.push_str(", world!");
    println!("{}", st); // hello, world! と表示

    let x = 5;
    let y = x;
    println!("x: {}, y: {}", x, y); // x: 5, y: 5 と表示

    let s1 = String::from("hello");
    let s2 = s1;
    // println!("s1: {}, s2: {}", s1, s2);
    // s1の所有権がs2に移ったので1つ上の操作はエラーになる。
    // Stringはヒープ領域に保存されるので、
    // メモリの使用を少なくするのと、 メモリ管理をしやすくするために、
    // ヒープ領域にあるデータは、 所有権が移譲される。
    // 代わって、int型やfloat型はデータサイズが決まっているので、
    // スタック領域に保存されるので、所有権の移譲は発生しない。
    println!("{}, world!", s2); // hello, world! と表示

    let s1 = String::from("hello");
    let s2 = s1.clone(); // ヒープ領域のデータをコピーするにはclone()を使う
    println!("s1: {}, s2: {}", s1, s2); // s1: hello, s2: hello と表示
}
