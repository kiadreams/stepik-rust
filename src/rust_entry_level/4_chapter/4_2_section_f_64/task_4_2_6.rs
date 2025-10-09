use std::io;

fn main() {
    let (mut str_kilo, mut str_pound) = (String::new(), String::new());
    io::stdin().read_line(&mut str_kilo).expect("msg");
    io::stdin().read_line(&mut str_pound).expect("msg");
    let kilo: f64 = str_kilo.trim().parse().expect("msg");
    let pound: f64 = str_pound.trim().parse().expect("msg");
    println!("{} kg = {:.3} lbs", kilo, kilo * 2.205);
    println!("{} lbs = {:.3} kg", pound, pound * 0.454);
}
