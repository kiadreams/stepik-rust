fn input(x: usize) -> Vec<u16> {
    let v = std::io::stdin()
        .lines()
        .take(x)
        .map(|v| v.unwrap().trim().parse::<u16>().unwrap())
        .collect::<Vec<_>>();
    return v;
}

fn main() {
    let [num] = input(1)[..] else { todo!() };
    let mut nums = vec![num / 100, num % 100 / 10, num % 10];
    nums.sort();
    let [x1, x2, x3] = nums[..] else { todo!() };
    println!("{x3}{x2}{x1}");
}
