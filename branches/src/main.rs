use std::io;
fn main() {
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("Failed  to read line.");
    let num: u32 = num.trim().parse().expect("Please type a string.");
    //let number = 6;
                                    // если не указать else прогрмма просто пойдет дальше
    if num % 4 == 0 {
        println!("number is divisible by 4");
    } else if num % 3 == 0 {
        println!("number is divisible by 3");
    } else if num % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

