fn main() {
    let error_in = "Что-то не так с вводом";
    let num_error_msg = "Требуется ввести число";
    let (mut str_num1, mut str_num2) = (String::new(), String::new());
    std::io::stdin().read_line(&mut str_num1).expect(error_in);
    std::io::stdin().read_line(&mut str_num2).expect(error_in);
    let num1: i32 = str_num1.trim().parse().expect(num_error_msg);
    let num2: i32 = str_num2.trim().parse().expect(num_error_msg);
    println!("{}", num1 + num2);
}
