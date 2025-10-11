fn input(x: usize) -> Vec<u32> {
    let v = std::io::stdin()
                    .lines()
                    .take(x)
                    .map(|v| v.unwrap().trim().parse::<u32>().unwrap())
                    .collect::<Vec<_>>();
    return v;
}

fn main() {
    let [x1] = input(1)[..] else { todo!() };
    println!(
        "Размер файла при повторной записи: {} Мбайт",
         x1 * 2 * 2 / (3 * 4)
    );
}
