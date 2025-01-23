use std::io;

fn main() {
    let arr = [-1, 0, 1, 2, 30, 4, 500];
    let mut str_in1 = String::new();
    io::stdin().read_line(&mut str_in1).expect("msg");
    let in1: usize = str_in1.trim().parse().expect("msg");
    println!("{}", arr[in1]);
}

