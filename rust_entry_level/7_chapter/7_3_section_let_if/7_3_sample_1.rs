fn main() {
    let cond = true;
    let num_1 = if cond { 3 + 7 } else { 5 };

    // "Значение переменной num = 10"
    // Важно, что значения, которые могут быть результатами каждого из ветвей if, должны быть одного типа:
    println!("Значение переменной num = {num_1}");

    // Ветки условий при этом не ограничиваются только одиночными числами. В них могут быть и другие выражения и инструкции:
    /*
    Обратите внимание, что строка x + 1 не имеет точки с запятой в конце. Выражения не содержат завершающих ;. Если вы добавите точку с запятой в конец выражения, вы сделайте из него инструкцию, и тогда оно не будет возвращать значение. Помните об этом при работе с выражениями.
     */
    // И последнее. При использовании if в качестве инициализатора блок else должен присутствовать обязательно!
    let flag = 0;
    let num_2 = if flag == 0 {
        let x = 10;
        x + 1
    } else {
        let x = 5;
        x + 1
    };
    println!("Значение переменной num = {num_2}");
}
