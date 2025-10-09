fn input() -> u16 {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("");
    return buf.trim().parse::<u16>().expect("");
}

fn main() {
    let num1 = input();
    let num2 = input();
    println!("Квартира с номером {} находится на {} этаже",
        num2,
        (num2 - 1) / num1 + 1
    );
}