fn input(x: usize) -> Vec<usize> {
    let v = std::io::stdin()
        .lines()
        .take(x)
        .map(|v| v.unwrap().trim().parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    return v;
}

fn main() {
    let array = [3, 1, 0, -5, -1, 300, 4, 2];
    let [min_value, max_value] = input(2)[..] else { todo!() };
    println!(
        "Считанный мин.индекс {}",
        if array[min_value] == -5 { "корректный" } else { "некорректный" }
    );
    println!(
        "Считанный макс.индекс {}",
        if array[max_value] == 300 { "корректный" } else { "некорректный" }
    );
}
