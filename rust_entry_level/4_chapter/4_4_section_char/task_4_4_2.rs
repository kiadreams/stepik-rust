use std::io;

fn main() {
    let mut s1 = String::new();
    let mut s2 = String::new();
    let mut s3 = String::new();
    let mut s4 = String::new();
    let mut s5 = String::new();
    io::stdin().read_line(&mut s1).expect("msg");
    io::stdin().read_line(&mut s2).expect("msg");
    io::stdin().read_line(&mut s3).expect("msg");
    io::stdin().read_line(&mut s4).expect("msg");
    io::stdin().read_line(&mut s5).expect("msg");
    let ch1: char = s1.trim().parse().expect("msg");
    let ch2: char = s2.trim().parse().expect("msg");
    let ch3: char = s3.trim().parse().expect("msg");
    let ch4: char = s4.trim().parse().expect("msg");
    let ch5: char = s5.trim().parse().expect("msg");
    println!("{}{}{}{}{}", ch1, ch2, ch3, ch4, ch5);
}
