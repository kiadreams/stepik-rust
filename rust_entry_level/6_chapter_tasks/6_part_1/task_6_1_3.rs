fn main() {
    let mut num = String::new();
    std::io::stdin().read_line(&mut num).expect("");
    let num: u16 = num.trim().parse().expect("");
    println!("{}", num / 100 + num % 100 / 10 + num % 10);
}