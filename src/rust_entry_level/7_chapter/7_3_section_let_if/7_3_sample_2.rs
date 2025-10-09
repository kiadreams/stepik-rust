fn main() {
    let flag = 0;
    let num = if flag == 0 {
        let x = 10;
        x + 1
    } else {
        let x = 10;
        x + 1
    };

    println!("Значение переменной num = {num}"); // "Значение переменной num = 11"
}