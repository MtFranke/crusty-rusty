use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Hello in guess game! Pick a number: ");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("error while reading!");
        println!("You entered: {}", guess);
        let guess:u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("To small!"),
            Ordering::Greater => println!("To big!"),
            Ordering::Equal => {
                println!("You win!!!");
                break;
            }
        }
    }
}