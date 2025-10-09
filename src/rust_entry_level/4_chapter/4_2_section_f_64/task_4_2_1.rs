use std::io;

fn main() {
    let (mut str_int, mut str_float) = (String::new(), String::new());
    io::stdin().read_line(&mut str_float).expect("msg");
    io::stdin().read_line(&mut str_int).expect("msg");
    let float: f64 = str_float.trim().parse().expect("msg");
    let int: usize = str_int.trim().parse().expect("msg");
    println!("{:.int$}", float);
}
