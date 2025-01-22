fn main() {
    let tup = (3, 0.1, 0.04, 0.001, 0.0005, 0.00009, 0.000002, 0.0000006);
    println!(
        "{}",
        tup.0 as f64 + tup.1 + tup.2 + tup.3 + tup.4 + tup.5 + tup.6 + tup.7,
    );
}
