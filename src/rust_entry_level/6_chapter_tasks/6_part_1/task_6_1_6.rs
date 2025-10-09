fn input() -> u64 {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("");
    return buf.trim().parse::<u64>().expect("");
}

fn main() {
    let s = input();
    println!(
        "{} сек = {} час {} минут {} секунд",
        s,
        s / 3600,
        s % 3600 / 60,
        s % 60 
    );
}