fn main() {
    let mut s = String::new();
    for _ in 0..10 {
        std::io::stdin().read_line(&mut s).expect("msg");
        println!("Вы ввели: {}", s.trim());
        s.clear();
    }
}