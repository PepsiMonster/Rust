fn main() {
    //let y = 6;                  // Инструкция (пример) => не возвращает значения 
    let x = 6;
    let z = five();
    let t = plus_one(6);

    let y = { // Пример выражения
        let x = 3;  // Все еще инструкция, т.к. есть ;
        x + 1       // Нет ; в конце, выражение => делает let y выражением 
    };

    another_function(x, y, z, t);         // Судя по всему cargo run запускает именно main
                                // Так что надо приписывать другие функции в main 
    // print_labeled_measurement(5,'h')

}

fn another_function(x: i32, y: i32, z: i32, t: i32) {   // Функции можно дать параметры
                                // Аргументы нужно явно задавать
    println!("The values are: {x}, {y}, {z}, {t}");
}
// fn print_labeled_measurement(value: i32, unit_label: char) {
//     println!("The measurement is: {value}{unit_label}");
// }


fn five() -> i32 {  // Мини функция
    5               
}

fn plus_one(x: i32) -> i32 {
    x + 1           // Если добавим ; будет ошибка т.к. x+1 станет инструкцией
}