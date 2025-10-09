fn main() {
    let mut number = String::new();
    std::io::stdin().read_line(&mut number).expect("");
    let number: u16 = number.trim().parse().expect("");
    println!("Последняя цифра: {}", number % 10);
    println!("Третья цифра: {}", number % 100 / 10);
    println!("Вторая цифра: {}", number % 1000 / 100);
    println!("Первая цифра: {}", number % 10000 / 1000);
}