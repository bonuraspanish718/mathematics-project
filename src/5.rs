fn main() {
    let mut num = 0;
    println!("Enter a number: ");
    io::stdin().read_line(&mut num);

    if num % 2 == 0 {
        println!("Even");
    } else {
        println!("Odd");
    }
}
