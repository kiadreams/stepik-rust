use std::io;

fn main() {
    let mut str_in1 = String::new();
    let mut str_in2 = String::new();
    let mut str_in3 = String::new();
    let mut str_in4 = String::new();
    let mut str_in5 = String::new();

    io::stdin().read_line(&mut str_in1).expect("msg");
    io::stdin().read_line(&mut str_in2).expect("msg");
    io::stdin().read_line(&mut str_in3).expect("msg");
    io::stdin().read_line(&mut str_in4).expect("msg");
    io::stdin().read_line(&mut str_in5).expect("msg");

    let some_tup = (str_in1, str_in2, str_in3, str_in4, str_in5);
    println!("{:?}", some_tup);
}

