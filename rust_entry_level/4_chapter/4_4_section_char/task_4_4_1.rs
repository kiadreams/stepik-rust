use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("msg");
    let some_char: char = s.trim().parse().expect("msg");
    println!("Мое настроение: {}", some_char);
}
