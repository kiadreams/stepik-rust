fn input() -> f64 {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("");
    return buf.trim().parse::<f64>().expect("");
}

fn main() {
    let lenth = [
        (0.621371, "миль"),
        (1093.61, "ярдов"),
        (3280.84, "футов"),
        (39370.1, "дюймов")
    ];
    let s = input();
    for (l, n) in lenth {
        println!("{} км = {:.3} {}", s, (l * s), n);
    }
}