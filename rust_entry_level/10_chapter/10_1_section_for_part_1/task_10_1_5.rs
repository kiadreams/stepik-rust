fn main() {
    let mut s_n = String::new();
    std::io::stdin().read_line(&mut s_n).expect("msg");
    let n: u16 = s_n.trim().parse().expect("msg");
    for i in 0..n {
        println!("{i}");
    }
}