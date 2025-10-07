use std::io;

fn main() {
    let (mut s_str, mut s_n1, mut s_n2) = (String::new(), String::new(), String::new());
    io::stdin().read_line(&mut s_str).expect("msg");
    io::stdin().read_line(&mut s_n1).expect("msg");
    io::stdin().read_line(&mut s_n2).expect("msg");
    let n1: i32 = s_n1.trim().parse().expect("msg");
    let n2: i32 = s_n2.trim().parse().expect("msg");
    for _ in n1..n2 {
        println!("{}", s_str.trim());
    }
}