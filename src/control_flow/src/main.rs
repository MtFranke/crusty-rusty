fn main() {
    let age = 0;
    if age >= 18 {
        println!("You can drink!");
    } else if age <= 0 {
        println!("Realy....");
    } else {
        println!("You are to young.")
    }

    let condition = false;
    let val = if condition { 5 } else { 6 };
    println!("{val}");

    let mut x = 0;
    while x < 10 {
        x += 1;
    }
    println!("End of loop, x on {x}");

    let a = [2, 4, 6, 8];
    for element in a {
        print!("{},", element);
    }

    for i in (1..100).rev() {
        print!("{i},");
    }
}
