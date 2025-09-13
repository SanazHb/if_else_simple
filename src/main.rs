fn main() {
    println!("Enter a number to check it:");
    let mut number: String = String::new();
    std::io::stdin().read_line(&mut number);
    let int_number: i32 = number.trim().parse().expect("ERROR");
    if int_number % 2 == 0 {
        println!("{int_number} is divisible by 2.");
    } else if int_number % 3 == 0 {
        println!("{int_number} is divisible by 3.");
    } else if int_number % 5 == 0 {
        println!("{int_number} is divisible by 5.");
    } else {
        println!("{int_number} is not divisible by 2,3,5.");
    }
}
