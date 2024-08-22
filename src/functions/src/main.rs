fn main() {
    another_function(5, 'h');
}

fn another_function(num: i32, c: char) {
    println!("Another function. {num}{c}");
    let val = add_numbers(3, 11);
    println!("{val}");
}

fn add_numbers(a: i32, b: i32) -> i32 {
    return a + b;
}
