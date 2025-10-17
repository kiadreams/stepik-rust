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
    let [mut x1, mut x2, mut x3] = nums[..] else { todo!() };
    if x1 == 0 && x2 == 0 {
        [x1, x3] = [x3, x1]
    } else if x1 == 0 {
        [x1, x2] = [x2, x1];
    }
    println!("{x1}{x2}{x3}");
}
