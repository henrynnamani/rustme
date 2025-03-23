fn main() {
    println!("{}", is_even(3));
}

pub fn is_even(number: u8) -> bool {
    let digit: u8 = number % 2;

    digit == 0
}