fn main() {
    let a = "X";
    {
        print!("{}", a);
    }
    print!("{}", a);

    let mut s = String::from("hello");
    s.push_str(", world");
    println!("{}", s);

    let x = 5;
    let y = x;
    println!("{x},{y}");

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("{s1}, {s2}");

    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
