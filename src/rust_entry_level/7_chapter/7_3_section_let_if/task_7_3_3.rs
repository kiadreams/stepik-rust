fn main() {
    let tup = (1, 3.14, -12.3, -50, 100, 250, -4, 7.6);
    println!(
        "{:.2}",
        tup.0 as f64 + tup.1 + tup.4 as f64 + tup.5 as f64 + tup.7
    );
}