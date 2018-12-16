fn main() {
    let a = [1, 2, 3, 4, 5];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index = index + 1;
    }

    for elment in a.iter() {
        println!("the value is : {}", elment);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
