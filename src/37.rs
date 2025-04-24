// Example Rust code for generating Fibonacci sequence up to n numbers.
fn main() {
    let mut sequence = vec![];
    let n: u32 = 10; // Change this value to generate a different number of terms.
    
    for i in 0..n {
        if i == 0 || i == 1 {
            sequence.push(0);
        } else {
            sequence.push(sequence[i - 1] + sequence[i - 2]);
        }
    }

    println!("Fibonacci sequence up to {} numbers: {:?}", n, sequence);
}
