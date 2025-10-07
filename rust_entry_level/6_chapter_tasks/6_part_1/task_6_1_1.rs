fn main() {
    let mut num = String::new();
    std::io::stdin().read_line(&mut num).expect("Считали");
    let num: i8 = num.trim().parse().expect("Это не число...");
    println!("{}\n{}", num / 10, num % 10);
}