use std::io;

fn main() {
    let (mut str_num1, mut str_num2) = (String::new(), String::new());
    io::stdin().read_line(&mut str_num1).expect("msg");
    io::stdin().read_line(&mut str_num2).expect("msg");
    let num1: i32 = str_num1.trim().parse().expect("msg");
    let num2: i32 = str_num2.trim().parse().expect("msg");
    println!("{0} x 1 = {1}", num1, num1 * 1);
    println!("{0} x 2 = {1}", num1, num1 * 2);
    println!("{0} x 3 = {1}", num1, num1 * 3);
    println!("{0} x 4 = {1}", num1, num1 * 4);
    println!("{0} x 5 = {1}", num1, num1 * 5);
    println!("{0} x 6 = {1}", num1, num1 * 6);
    println!("{0} x 7 = {1}", num1, num1 * 7);
    println!("{0} x 8 = {1}", num1, num1 * 8);
    println!("{0} x 9 = {1}", num1, num1 * 9);
    println!("{0} x 10 = {1}\n", num1, num1 * 10);

    println!("{num2} x 1 = {}", num2 * 1);
    println!("{num2} x 2 = {}", num2 * 2);
    println!("{num2} x 3 = {}", num2 * 3);
    println!("{num2} x 4 = {}", num2 * 4);
    println!("{num2} x 5 = {}", num2 * 5);
    println!("{num2} x 6 = {}", num2 * 6);
    println!("{num2} x 7 = {}", num2 * 7);
    println!("{num2} x 8 = {}", num2 * 8);
    println!("{num2} x 9 = {}", num2 * 9);
    println!("{num2} x 10 = {}", num2 * 10);
}
