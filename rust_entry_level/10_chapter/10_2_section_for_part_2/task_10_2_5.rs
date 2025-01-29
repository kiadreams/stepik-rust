fn main() {
    let mut s_n = String::new();
    let mut arr = [0i32; 3];
    let mut multiple = 1f64;
    for i in 0..3 {
        std::io::stdin().read_line(&mut s_n).expect("msg");
        arr[i] = s_n.trim().parse().expect("msg");
        s_n.clear();
    }
    let [start, end, step] = arr;
    for i in (start..=end).step_by(step as usize) {
        multiple *= i as f64;
    }
    println!("{:.1}", multiple);
}