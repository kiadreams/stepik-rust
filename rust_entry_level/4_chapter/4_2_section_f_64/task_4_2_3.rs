use std::io;

fn main() {
    let mut str_value = String::new();
    io::stdin().read_line(&mut str_value).expect("msg");
    let value: f64 = str_value.trim().parse().expect("msg");
    let int_value = value as i32;
    println!("{}\n{:.3}", int_value, value - int_value as f64);
}
