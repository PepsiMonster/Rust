use std::io;
fn main() {
    let mut faren = String::new();
    io::stdin().read_line(&mut faren).expect("Failed  to read line.");
    let faren: f64 = faren.trim().parse().expect("Please type a number.");
    //println!("{faren}");
    let celc: f64 = (5.0/9.0) * (faren-32.0);
    println!("{celc}");
}