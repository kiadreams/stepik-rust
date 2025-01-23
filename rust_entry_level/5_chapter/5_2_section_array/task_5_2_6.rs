use std::io;

fn main() {
    let arr = [-5, 1, 8, 2, 30, 4000, 500000];
    let mut str_i = String::new();
    io::stdin().read_line(&mut str_i).expect("msg");
    let i: usize = str_i.trim().parse().expect("msg");
    println!(
        "{}\n{}\n{}",
        arr[i - 1] + arr[i + 1],
        arr[i - 1] - arr[i + 1],
        arr[i - 1] * arr[i + 1],
    );
}
