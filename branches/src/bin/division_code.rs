use std::io;
fn main() {
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("Failed  to read line.");
    let num: u32 = num.trim().parse().expect("Please type a string.");
    //let number = 6;
                                    // если не указать else прогрмма просто пойдет дальше
    let mut is_divisible = false; // Флаг - было ли хоть одно деление
    
    if num % 4 == 0 {
        println!("number is divisible by 4");
        is_divisible = true;
    }
    
    if num % 3 == 0 {
        println!("number is divisible by 3");
        is_divisible = true;
    }
    
    if num % 2 == 0 {
        println!("number is divisible by 2");
        is_divisible = true;
    }
    
    if !is_divisible { // Если ни на что не делится
        println!("number is not divisible by 4, 3, or 2");
    }
}
