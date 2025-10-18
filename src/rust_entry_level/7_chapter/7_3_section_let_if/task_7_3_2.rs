fn input(x: usize) -> Vec<u8> {
    let v = std::io::stdin()
        .lines()
        .take(x)
        .map(|v| v.unwrap().trim().parse::<u8>().unwrap())
        .collect::<Vec<_>>();
    return v;
}

fn main() {
    let [month] = input(1)[..] else { todo!() };
    let season: &str = if month < 3 || month == 12 {
        "Зима"
    } else if month < 6 {
        "Весна"
    } else if month < 9 {
        "Лето"
    } else {
        "Осень"
    };
    println!("{season}");
}
