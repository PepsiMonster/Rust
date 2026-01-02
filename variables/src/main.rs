// переменные, создаваемые через let (пусть x =5) по умолчанию неизменяемые
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

