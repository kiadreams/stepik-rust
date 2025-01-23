use std::io;

fn main() {
    let mut arr = [-621.5, 11.1, 2.0, -7.123, 0.125, 0.0, 0.000051789];
    let (mut str_i1, mut str_i2) = (String::new(), String::new());
    io::stdin().read_line(&mut str_i1).expect("msg");
    io::stdin().read_line(&mut str_i2).expect("msg");
    let i1: usize = str_i1.trim().parse().expect("msg");
    let i2: usize = str_i2.trim().parse().expect("msg");
    (arr[i1], arr[i2]) = (arr[i2], arr[i1]);
    println!("{:.9?}", arr);
}
