fn main(){
    // let mut num1: i32 = 0;
    // let mut num2: i32 = 1;
    // let mut summ: i32 = 0;
    // let mut n:i32 = 1;
    // while n <=10 {
    //     summ = num1+num2;
    //     println!("The {} fibonacci number is {}", n, summ);
    //     num1 = num2;
    //     num2 = summ;
    //     n += 1;
    // }
    let mut num1: i32 = 0;
    let mut num2: i32 = 1;
    let mut summ: i32 = 0;
    for n in 0..10 {
        println!("The {} fibonacci number is {}", n, num1);
        let summ: i32 = num1+num2;
        num1 = num2;
        num2 = summ;
    }
}

