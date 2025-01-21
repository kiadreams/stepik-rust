use std::io;

fn main() {
    let mut str_value = String::new();
    io::stdin().read_line(&mut str_value).expect("msg");
    let value: f64 = str_value.trim().parse().expect("msg");
    println!("{:E}", value);
}
