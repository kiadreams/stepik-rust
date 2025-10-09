use std::io;

fn main() {
    let (mut str_num1, mut str_num2) = (String::new(), String::new());
    io::stdin().read_line(&mut str_num1).expect("msg");
    io::stdin().read_line(&mut str_num2).expect("msg");
    let num1: f64 = str_num1.trim().parse().expect("msg");
    let num2: f64 = str_num2.trim().parse().expect("msg");
    println!("{num1} + ({num2}) = {}", num1 + num2);
    println!("{num1} - ({num2}) = {}", num1 - num2);
    println!("{num1} * ({num2}) = {}", num1 * num2);
    println!("{num1} / ({num2}) = {:.3}", num1 / num2);
    println!("{num1} % ({num2}) = {:.3}", num1 % num2);
}
