fn main() {
    let mut num = String::new();
    std::io::stdin().read_line(&mut num).expect("Считали");
    let num: f64 = num.trim().parse().expect("Это не число...");
    println!("{:.3}", num % 1.0);
}