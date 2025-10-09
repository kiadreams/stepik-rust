use std::io;

fn main() {
    let (mut str_balans, mut str_money) = (String::new(), String::new());
    io::stdin().read_line(&mut str_balans).expect("msg");
    io::stdin().read_line(&mut str_money).expect("msg");
    let balans: f64 = str_balans.trim().parse().expect("msg");
    let money: u32 = str_money.trim().parse().expect("msg");
    println!("{:.1}", balans + money as f64);
}
