fn main() {
    let mut num = String::new();
    std::io::stdin()
        .read_line(&mut num)
        .expect("msg");
    let num_1: i32 = num
        .trim()
        .parse()
        .expect("msg num");
    println!("{num_1:#b}\n{num_1:#o}\n{num_1:#x}");
}