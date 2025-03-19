use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign};

fn main() {
    let mut a = 1;
    let mut b = 2;
    let mut c = 3;

    // Addition assignment operator
    a += b;
    println!("{}", a); // Output: 3

    // Subtraction assignment operator
    a -= b;
    println!("{}", a); // Output: 1

    // Multiplication assignment operator
    c *= b;
    println!("{}", c); // Output: 6

    // Division assignment operator
    a /= b;
    println!("{}", a); // Output: 0.5
}
