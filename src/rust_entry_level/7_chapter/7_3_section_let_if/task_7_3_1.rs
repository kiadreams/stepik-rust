use std::io;


fn main() {
    let arr = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut str_month_num = String::new();
    io::stdin().read_line(&mut str_month_num).expect("msg");
    let month_num: usize = str_month_num.trim().parse().expect("msg");
    println!("{}", arr[month_num - 1]);
}