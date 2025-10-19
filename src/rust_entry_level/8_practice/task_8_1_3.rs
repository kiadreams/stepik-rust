fn input(x: usize) -> Vec<i16> {
    let v = std::io::stdin()
        .lines()
        .take(x)
        .map(|v| v.unwrap().trim().parse::<i16>().unwrap())
        .collect::<Vec<_>>();
    return v;
}

fn main() {
    let letters = ['О', 'Н', 'Г', 'И'];
    let [n] = input(1)[..] else { todo!() };
    let x1 = (n / 1000) as usize - 1;
    let x2 = (n / 100 % 10) as usize - 1;
    let x3 = (n / 10 % 10) as usize - 1;
    let x4 = (n % 10) as usize - 1;
    println!("{}{}{}{}", letters[x1], letters[x2], letters[x3], letters[x4]);
}
