use std::{thread, time};

fn main() {
    let five = 5;
    let seven = 7;
    let tup = (five, seven);
    thread::sleep(time::Duration::from_secs(1));
}
