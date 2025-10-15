fn input(x: usize) -> Vec<i64> {
    let v = std::io::stdin()
        .lines()
        .take(x)
        .map(|v| v.unwrap().trim().parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    return v;
}

fn main() {
    let arr = [
        -2.5, 4.2, 9.1, 22.5, 30.0, 1445.123, 1000000.0, 0.001, 0.5, -0.127,
    ];

    let [ind] = input(1)[..] else { todo!() };
    if ind < 0 {
        println!("Отрицательный индекс приводит к панике");
    }
    if ind > 9 {
        println!("Попытка выхода за пределы массива");
    }
    if ind >= 0 && ind < 10 {
        println!("Элемент с индексом {} равен {:.3}", ind, arr[ind as usize]);
    }
}
