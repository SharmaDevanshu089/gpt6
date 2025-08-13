fn main() {
    let number = 5;
    println!("Multipy by 2 is {}", multipy_by_2(number));
    print!("Inital number was {}", number);
}
fn multipy_by_2(intial_number : i32) -> i32 {
    let new_number = &intial_number *2;
    return new_number;
}