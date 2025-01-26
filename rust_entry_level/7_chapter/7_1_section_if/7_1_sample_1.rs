fn main() {
    let num_1 = 10;

    let ex_res_1 = num_1 > 5;
    let ex_res_2 = 3 > 5;
    if ex_res_1 {
        println!("Число {num_1} > 5 - {}", ex_res_1);
    }
    // Но одновременно с этим...
    if ex_res_2 {
        println!("Условие не выполняется... поэтому это не выведется");
    } else {
        println!("Выводим блок else...");
    }
}