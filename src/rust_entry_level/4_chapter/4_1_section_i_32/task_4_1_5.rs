use std::io;

fn main() {
    let (mut str_num1, mut str_num2) = (String::new(), String::new());
    io::stdin().read_line(&mut str_num1).expect("msg");
    io::stdin().read_line(&mut str_num2).expect("msg");
    let mut num1: i32 = str_num1.trim().parse().expect("msg");
    let mut num2: i32 = str_num2.trim().parse().expect("msg");
    (num1, num2) = (num2, num1);
    println!("{num1}\n{num2}");
}
